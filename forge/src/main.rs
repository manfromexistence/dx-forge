use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

// --- FFI Declaration ---
// This block tells Rust about the function we exported from our Zig library.
// The `link` attribute tells the linker to include `libgenerator.a`.
#[link(name = "generator", kind = "static")]
extern "C" {
    /// This function is defined in `src/generator.zig` and compiled by `build.rs`.
    fn process_tsx_file(path_ptr: *const u8, path_len: usize) -> bool;
}

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

    let path_to_watch = ".";
    println!("DX Observer: Watching directory -> {}", Path::new(path_to_watch).canonicalize()?.display());

    let (tx, mut rx) = mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                if tx.try_send(event).is_err() {
                    eprintln!("Warning: Channel is full, event dropped.");
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(Path::new(path_to_watch), RecursiveMode::Recursive)?;

    println!("DX Observer: Watcher is active. Waiting for file changes...");
    println!("(Try creating/modifying a .tsx file with a <Dx.IconName /> tag)");

    while let Some(event) = rx.recv().await {
        // We only care about creation or modification events.
        if event.kind.is_modify() || event.kind.is_create() {
            let tsx_paths: Vec<_> = event
                .paths
                .into_iter()
                .filter(|path| path.extension().map_or(false, |ext| ext == "tsx"))
                .collect();

            if !tsx_paths.is_empty() {
                println!("\n[TSX EVENT DETECTED] >> {:?}", event.kind);
                for path in tsx_paths {
                    println!("  -> Path: {}", path.display());

                    // ==========================================================
                    //  !!! CALLING THE ZIG GENERATOR !!!
                    // ==========================================================
                    if let Some(path_str) = path.to_str() {
                        // Calling an external C function is `unsafe` because Rust's
                        // compiler cannot guarantee its memory safety. We trust our Zig code.
                        let generated = unsafe {
                            process_tsx_file(path_str.as_ptr(), path_str.len())
                        };

                        // *** NEW: Check the result from Zig and report it! ***
                        if generated {
                            println!("  -> ACTION: Zig generator successfully created a new component.");
                        } else {
                            println!("  -> INFO: No <Dx.*> component found to generate in this change.");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
