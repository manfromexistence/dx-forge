//! # DX - The Developer Experience Toolkit
//!
//! This is the main entry point for the `dx` binary. Its primary role
//! is to initialize the necessary modules and start the core application logic.

// Declare our new modules so Rust knows they exist.
// It will automatically look for src/generator/mod.rs and src/observer/mod.rs
mod generator;
mod observer;

/// The main function for the application.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DX: Initializing...");

    // Start the observer module's main loop.
    if let Err(e) = observer::start().await {
        eprintln!("DX Error: The observer failed with an error: {}", e);
    }

    println!("DX: Shutting down.");
    Ok(())
}