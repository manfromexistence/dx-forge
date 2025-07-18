//! # Observer Module
//!
//! This module contains the core file-watching logic for the `dx` tool.
//! It uses the `notify` crate to listen for file system events asynchronously
//! and dispatches them for processing by the generator.

use crate::generator; // Use our new, safe generator module
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;

/// Starts the asynchronous file observer.
///
/// This function sets up a file watcher for the current directory and listens
/// for changes to `.tsx` files. When a change is detected, it calls the
/// generator module to process the file.
pub async fn start() -> anyhow::Result<()> {
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

                    if let Some(path_str) = path.to_str() {
                        // Call the safe wrapper in our generator module.
                        let generated = generator::process_file(path_str);

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
