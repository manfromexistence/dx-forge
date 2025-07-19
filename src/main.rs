//! # DX - The Developer Experience Toolkit
//!
//! This is the main entry point for the `dx` binary. Its primary role
//! is to initialize the necessary modules and start the core application logic.

// Declare our modules so Rust knows they exist.
mod chronicle;
mod generator;
mod observer;

/// The main function for the application.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DX: Initializing...");

    // --- Initialize the Chronicle ---
    // This will create the .dx/chronicle directory and git repo on the first run.
    let chronicle_repo = match chronicle::initialize() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("DX Error: Failed to initialize the Chronicle: {}", e);
            // We exit if we can't create our safety net.
            return Err(e);
        }
    };

    // Start the observer module's main loop.
    // We pass a clone of the repository handle to the observer.
    if let Err(e) = observer::start(chronicle_repo.clone()).await {
        eprintln!("DX Error: The observer failed with an error: {}", e);
    }

    println!("DX: Shutting down.");
    Ok(())
}
