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

<!-- 
use lolcrab::Lolcrab;
use std::io;

const TEXT: &str = "\
•••••••••••••••••••••••••••••••••••••••••••
••442463299144744830108724702438783348716••
••665891426009540978622724448305819269356••
••078289454141226451790882961903610719673••
••56505384476•••••••••••••••••39761609699••
••47928752907•• { lolcrab } ••33810561851••
••51609982385•••••••••••••••••43459368213••
••980457234663167653959566555465520046709••
••677103598707232478714861999441705454744••
••012721882924436718718457599087686681354••
•••••••••••••••••••••••••••••••••••••••••••
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // Initialize Lolcrab using default gradient and default noise
    let mut lol = Lolcrab::new(None, None);

    lol.colorize_str(TEXT, &mut stdout)?;

    lol.set_invert(true);
    lol.randomize_position();
    lol.colorize_str(TEXT, &mut stdout)?;

    lol.set_invert(false);
    lol.reset_position();
    lol.colorize_str(TEXT, &mut stdout)?;

    Ok(())
} -->

<!-- 
use figlet_rs::FIGfont;
use lolcrab::Lolcrab;
use std::io::{self, Write};

fn main() {
    // 1. Create the big text with figlet
    let font = FIGfont::standard().unwrap();
    let figlet_text = font.convert("dx").unwrap();
    let figlet_string = figlet_text.to_string();

    // 2. Initialize Lolcrab
    let mut lol = Lolcrab::new(None, None);

    // 3. Get a handle to the terminal output
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // 4. Clear the screen to ensure a clean display
    write!(handle, "\x1B[2J\x1B[1;1H").unwrap();

    // 5. Colorize the text and print it to the terminal just once
    lol.colorize_str(&figlet_string, &mut handle).unwrap();
} 
-->


<!-- 
use dx::Text;

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
} 
-->
