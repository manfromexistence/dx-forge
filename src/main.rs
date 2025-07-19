mod chronicle;
mod generator;
mod observer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DX: Initializing...");

    let chronicle_repo = match chronicle::initialize() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("DX Error: Failed to initialize the Chronicle: {}", e);
            return Err(e);
        }
    };

    if let Err(e) = observer::start(chronicle_repo.clone()).await {
        eprintln!("DX Error: The observer failed with an error: {}", e);
    }

    println!("DX: Shutting down.");
    Ok(())
}