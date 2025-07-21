// main.rs

// To run this code, you need to add the `rayon` crate for easy parallelism.
// Add this line to your `Cargo.toml` file under `[dependencies]`:
// rayon = "1.8.1"
// Then, run the code in release mode for maximum optimization: `cargo run --release`

use rayon::prelude::*; // Import the parallel iterator traits
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    // --- Configuration ---
    const NUM_FILES: u32 = 100;
    const DIRECTORY_NAME: &str = "rust_modules";
    const FILE_CONTENT: &[u8] = b"Hello, manfromexistence!";

    println!(
        "Mission starting: Creating {} files in the '{}' directory...",
        NUM_FILES, DIRECTORY_NAME
    );

    // --- Setup ---
    // Create the target directory. `create_dir_all` is like `mkdir -p`,
    // it won't return an error if the directory already exists.
    fs::create_dir_all(DIRECTORY_NAME).expect("Failed to create directory.");

    // --- Timed Operation ---
    let start_time = Instant::now();

    // Use Rayon's parallel iterator to create files concurrently.
    // `into_par_iter()` converts the range into a parallel iterator.
    // `for_each` will then execute the provided closure on multiple threads.
    (0..NUM_FILES).into_par_iter().for_each(|i| {
        // Create a path for each new file. Using PathBuf is the idiomatic way.
        let mut file_path = PathBuf::from(DIRECTORY_NAME);
        file_path.push(format!("file_{}.txt", i));

        // Create the file.
        let mut file =
            File::create(&file_path).expect(&format!("Failed to create file: {:?}", file_path));

        // Write the content to the file.
        file.write_all(FILE_CONTENT)
            .expect(&format!("Failed to write to file: {:?}", file_path));
    });

    // --- Report Results ---
    let duration = start_time.elapsed();

    println!("\n--- Mission Accomplished, manfromexistence! ---");
    println!("Successfully created {} files.", NUM_FILES);
    println!("Total time taken: {:.2?}", duration);
    println!("-----------------------------------------------");
}
