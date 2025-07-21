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