use crate::cli::Text;

fn main() {
    let name = Text::new("What command you want to run?").prompt();

    match name {
        Ok(name) => println!("Command [{name}] is still in developement - it is coming soon..."),
        Err(_) => println!("An error happened when running this command, try again later."),
    }
}

// mod chronicle;
// mod generator;
// mod observer;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     println!("DX: Initializing...");

//     let chronicle_repo = match chronicle::initialize() {
//         Ok(repo) => repo,
//         Err(e) => {
//             eprintln!("DX Error: Failed to initialize the Chronicle: {}", e);
//             return Err(e);
//         }
//     };

//     if let Err(e) = observer::start(chronicle_repo.clone()).await {
//         eprintln!("DX Error: The observer failed with an error: {}", e);
//     }

//     println!("DX: Shutting down.");
//     Ok(())
// }
