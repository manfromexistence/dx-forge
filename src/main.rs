use figlet_rs::FIGfont;
use lolcrab::Lolcrab;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Create the big text with figlet
    let font = FIGfont::standard().unwrap();
    let figlet_text = font.convert("dx project").unwrap();
    let figlet_string = figlet_text.to_string();

    // 2. Initialize Lolcrab from v0.4.1
    let mut lol = Lolcrab::new(None, None);

    // Get a handle to the terminal output
    let stdout = io::stdout();

    // 3. Start the animation loop
    loop {
        let mut handle = stdout.lock();

        // Clear the screen and move the cursor to the top
        write!(handle, "\x1B[2J\x1B[1;1H").unwrap();

        // Colorize the text and print it directly to the terminal
        lol.colorize_str(&figlet_string, &mut handle).unwrap();

        // Ensure the output is written immediately
        handle.flush().unwrap();

        // Randomize the color position for the next frame's animation
        lol.randomize_position();

        // Pause briefly to control animation speed
        thread::sleep(Duration::from_millis(50));
    }
}