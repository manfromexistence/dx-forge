use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

/// # Asynchronous File Watcher
///
/// This is the main entry point for our file observer. It uses the `tokio` runtime
/// to run asynchronously. The primary job is to watch a directory for changes
/// and dispatch events for processing.
///
/// @returns A `Result` which is `Ok(())` on successful execution or an `Err`
///          containing an `anyhow::Error` if something goes wrong.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DX Observer: Initializing...");

    // The path to watch. We'll start by watching the current working directory.
    let path_to_watch = ".";
    println!("DX Observer: Watching directory -> {}", Path::new(path_to_watch).canonicalize()?.display());

    // --- Channel for Communication ---
    // We create a channel to communicate between the file watcher thread and our main async task.
    // `notify` sends events on one thread, and we want to process them in our main `tokio` task
    // without blocking. An `mpsc` (multi-producer, single-consumer) channel is perfect for this.
    let (tx, mut rx) = mpsc::channel(100);

    // --- The File Watcher ---
    // We create an instance of the `RecommendedWatcher`, which automatically selects the best
    // backend for the current operating system (e.g., inotify on Linux, FSEvents on macOS).
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            // This is the event handler closure. It's called by `notify` when an event occurs.
            // We don't block here. We just send the event to our main task through the channel.
            // `try_send` is non-blocking. If the channel is full, it returns an error,
            // which is fine for this example.
            if let Ok(event) = res {
                if tx.try_send(event).is_err() {
                    // This can happen if the receiver is lagging. For a real app, you might want
                    // to log this or use a different channel strategy (e.g., unbounded).
                    eprintln!("Warning: Channel is full, event dropped.");
                }
            }
        },
        Config::default(),
    )?;

    // Start watching the specified path. `RecursiveMode::Recursive` means it will
    // watch all subdirectories as well.
    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    println!("DX Observer: Watcher is active. Waiting for file changes...");
    println!("(Try creating, modifying, or deleting a .tsx file)");

    // --- Event Processing Loop ---
    // This loop will run forever, waiting for events to arrive on the channel's
    // receiving end (`rx`).
    while let Some(event) = rx.recv().await {
        // We only care about events that indicate a change to the content.
        // We can ignore metadata-only changes for now.
        if !event.kind.is_access() && !event.kind.is_other() {
            // Filter the paths to only include those with a .tsx extension.
            let tsx_paths: Vec<_> = event
                .paths
                .into_iter()
                .filter(|path| path.extension().map_or(false, |ext| ext == "tsx"))
                .collect();

            // Only proceed if we found at least one .tsx file in the event.
            if !tsx_paths.is_empty() {
                println!("\n[TSX EVENT DETECTED] >> {:?}", event.kind);
                for path in tsx_paths {
                    println!("  -> Path: {}", path.display());

                    // ==========================================================
                    //  !!! STAGE 2: CALL THE ZIG GENERATOR !!!
                    //  This is where we will use FFI to call our super-fast
                    //  Zig code to process the changed file.
                    //
                    //  For now, we'll just print a placeholder message.
                    // ==========================================================
                    println!("  -> ACTION: Queued for processing by Zig generator.");
                }
            }
        }
    }

    Ok(())
}
