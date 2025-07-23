// main.rs

// Import necessary modules from the standard library for file system operations,
// I/O (including buffered writing), and timing.
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

// Import the parallel iterator traits from the rayon crate.
// This is the key to executing our file creation tasks in parallel.
use rayon::prelude::*;

// The main function, which is the entry point of the program.
// It returns a Result to handle potential I/O errors gracefully.
fn main() -> io::Result<()> {
    println!("Jarvis mode activated. Starting high-speed file creation for manfromexistence...");

    // Define the name of the directory where files will be created.
    let dir_name = "rust_modules";
    // Define the number of files to create.
    let num_files = 1000;
    // Define the content to be written into each file.
    let content = "Hello, manfromexistence";

    // --- Setup Phase ---
    // Create the target directory. `create_dir_all` is used because it
    // won't return an error if the directory already exists.
    fs::create_dir_all(dir_name)?;
    println!("Directory '{}' is ready.", dir_name);

    // Start a timer to measure the execution time of the file creation process.
    let start_time = Instant::now();

    // --- Parallel Execution Phase ---
    // Create a range from 0 up to num_files (e.g., 0 to 999).
    // .par_iter() converts this range into a parallel iterator.
    // Rayon's scheduler will intelligently divide the work among multiple threads.
    (0..num_files).into_par_iter().for_each(|i| {
        // This closure will be executed in parallel for each number `i` in the range.
        
        // Construct the full path for the new file.
        // e.g., "rust_modules/file_0.txt", "rust_modules/file_1.txt", etc.
        let path = format!("{}/file_{}.txt", dir_name, i);

        // The file creation and writing logic is wrapped in a function
        // to handle potential errors for each individual file.
        // We use a `match` block to handle the Result. If an error occurs,
        // we print it to the console but don't stop the entire program.
        match create_and_write_file(&path, content) {
            Ok(_) => (), // If successful, do nothing.
            Err(e) => eprintln!("Failed to create file {}: {}", path, e), // If error, print it.
        }
    });

    // --- Completion Phase ---
    // Stop the timer and calculate the elapsed duration.
    let duration = start_time.elapsed();

    println!("\nTask complete!");
    println!("Successfully initiated the creation of {} files.", num_files);
    println!("Total time taken: {:?}", duration);

    // Return Ok to indicate that the main function completed successfully.
    Ok(())
}

/// Creates a file at the specified path and writes content to it using a BufWriter.
///
/// # Arguments
/// * `path` - The full path where the file will be created.
/// * `content` - The string slice to write into the file.
///
/// # Returns
/// * `io::Result<()>` - An empty Ok on success, or an io::Error on failure.
fn create_and_write_file(path: &str, content: &str) -> io::Result<()> {
    // Create the file. The `?` operator will propagate any errors.
    let file = File::create(path)?;
    
    // Wrap the file handle in a BufWriter. This creates an in-memory buffer.
    // Writes will go to this buffer first, and the buffer will be "flushed"
    // to the actual file on disk in larger, more efficient chunks.
    // This significantly reduces the number of slow system calls.
    let mut writer = BufWriter::new(file);
    
    // Write the content to the buffer.
    writer.write_all(content.as_bytes())?;
    
    // The BufWriter is automatically flushed when it goes out of scope at the
    // end of this function, ensuring all buffered data is written to the disk.
    Ok(())
}

// --- How to Run This Code ---
// 1. Make sure you have Rust installed.
// 2. Create a new Rust project: `cargo new fast_file_creator`
// 3. `cd fast_file_creator`
// 4. Add the `rayon` dependency to your `Cargo.toml` file:
//
// [dependencies]
// rayon = "1.8.1"
//
// 5. Replace the content of `src/main.rs` with this code.
// 6. Run the project from your terminal: `cargo run --release`
//    (The --release flag enables optimizations, making it even faster!)
