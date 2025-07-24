// main.rs

// --- Parallelism Dependencies ---
// Import the parallel iterator traits from the rayon crate.
// The prelude includes the most common and useful traits, like `ParallelIterator`.
use rayon::prelude::*;

// --- Standard Library ---
// Import necessary modules for file system operations, I/O, and timing.
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

// The main function is a standard synchronous function.
// It returns a Result to handle potential I/O errors gracefully.
fn main() -> io::Result<()> {
    println!("Jarvis mode activated. Re-calibrating for CPU-bound parallelism with Rayon...");

    // --- Configuration ---
    let dir_name = "rust_modules";
    let num_files = 1000;
    let content = "Hello, manfromexistence";

    // --- Setup Phase ---
    // Create the target directory. `create_dir_all` won't error if it already exists.
    fs::create_dir_all(dir_name)?;
    println!("Directory '{}' is ready.", dir_name);

    let start_time = Instant::now();

    // --- Parallel Execution Phase ---
    // Create a range from 0 to 999.
    // `.into_par_iter()` converts this range into a parallel iterator. Rayon's
    // scheduler will intelligently divide the work among multiple threads.
    // `try_for_each` is used for operations that can fail, stopping all threads
    // if any single operation returns an error.
    let result: io::Result<()> = (0..num_files).into_par_iter().try_for_each(|i| {
        // This closure will be executed in parallel for each number `i`.
        let path = format!("{}/file_{}.txt", dir_name, i);
        create_and_write_file(&path, content)
    });

    // Check the result of the entire parallel operation after it completes.
    // The `?` operator will propagate the first error that occurred, if any.
    result?;

    // --- Completion Phase ---
    let duration = start_time.elapsed();

    println!("\nTask complete!");
    println!("Parallel creation of {} files successful.", num_files);
    println!("Total time taken: {:?}", duration);

    Ok(())
}

/// Creates a file at the specified path and writes content to it using a BufWriter.
/// This is a standard, synchronous function.
fn create_and_write_file(path: &str, content: &str) -> io::Result<()> {
    // Create the file. The `?` operator will propagate any errors.
    let file = File::create(path)?;
    
    // Wrap the file handle in a BufWriter for efficient, buffered I/O.
    let mut writer = BufWriter::new(file);
    
    // Write the content to the buffer.
    writer.write_all(content.as_bytes())?;
    
    // The BufWriter is automatically flushed when it goes out of scope here.
    Ok(())
}

// --- How to Run This Code ---
// 1. Make sure you have Rust installed.
// 2. Create a new Rust project: `cargo new parallel_file_creator`
// 3. `cd parallel_file_creator`
// 4. Add the `rayon` dependency to your `Cargo.toml` file with the specified version.
//
// [dependencies]
// rayon = "1.10.0"
//
// 5. Replace the content of `src/main.rs` with this code.
// 6. Run the project from your terminal with optimizations: `cargo run --release`
