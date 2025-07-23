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
    println!("Jarvis mode activated. Engaging hyper-speed protocols for manfromexistence...");

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
    // We now use `try_for_each` which is designed for operations that can fail.
    // It will run the operations in parallel and stop if any of them return an `Err`.
    // This is more idiomatic and can be more efficient for error handling than a manual match.
    let result: io::Result<()> = (0..num_files).into_par_iter().try_for_each(|i| {
        // Construct the full path for the new file.
        let path = format!("{}/file_{}.txt", dir_name, i);
        
        // The file creation and writing function is called directly.
        // If it returns an Err, `try_for_each` will handle propagating it.
        create_and_write_file(&path, content)
    });

    // We check the result of the entire parallel operation *after* it completes.
    // The `?` operator will propagate the first error that occurred, if any.
    result?;

    // --- Completion Phase ---
    // Stop the timer and calculate the elapsed duration.
    let duration = start_time.elapsed();

    println!("\nTask complete!");
    println!("Successfully created {} files with enhanced protocols.", num_files);
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
    let mut writer = BufWriter::new(file);
    
    // Write the content to the buffer.
    writer.write_all(content.as_bytes())?;
    
    // The BufWriter is automatically flushed when it goes out of scope.
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
