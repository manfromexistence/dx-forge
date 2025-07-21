# dx
https://github.com/manfromexistence/dx-forge.git
find . -maxdepth 1 -mindepth 1 -type d -exec du -sh {} + | sed 's/K/KB/; s/M/MB/; s|\./||'
find . -maxdepth 1 -mindepth 1 ! -name "cli" ! -name "src" ! -name "creates" ! -name "packages" -exec rm -rf {} +
npm install -g @anthropic-ai/claude-code


```
git clone https://github.com/anthropics/claude-code && cd claude-code && rm -rf .git && cd ..
git clone https://github.com/ratatui/ratatui && cd ratatui && rm -rf .git && cd ..
git clone https://github.com/google-gemini/gemini-cli && cd gemini-cli && rm -rf .git && cd ..
git clone https://github.com/mikaelmello/inquire && cd inquire && rm -rf .git && cd ..
git clone https://github.com/bombshell-dev/clack && cd clack && rm -rf .git && cd ..
git clone https://github.com/oven-sh/bun && cd bun && rm -rf .git && cd ..
git clone https://github.com/haydenbleasel/ultracite.git && cd ultracite && rm -rf .git && cd ..
git clone https://github.com/tailwindlabs/tailwindcss && cd tailwindcss && rm -rf .git && cd ..
```


<!-- use dx::Text;

fn main() {
    let name = Text::new("What command you want to run?").prompt();

    match name {
        Ok(name) => println!("Command [{name}] is still in developement - it is coming soon..."),
        Err(_) => println!("An error happened when running this command, try again later."),
    }
}

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
} -->
