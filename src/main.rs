use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};
use std::time::Instant; // Import the Instant struct for timing.

// This is the line I've added. It explicitly tells the Rust linker
// to link against the static library named "file_creator_lib"
// which our `build.rs` script creates.
#[link(name = "file_creator_lib", kind = "static")]
extern "C" {
    // This `extern` block is the Foreign Function Interface (FFI) boundary.
    // It tells Rust about the function signature from our C library.
    fn create_files_in_c(dir_name: *const c_char, file_count: c_int, content: *const c_char) -> c_double;
}

fn main() {
    // --- Start of Rust-side timing ---
    let rust_start_time = Instant::now();

    // Define the parameters for our C function in Rust.
    let dir_name = "c_modules";
    let file_count = 100;
    let content = "Hello, manfromexistence!";

    // To pass strings to C, they must be null-terminated.
    // `CString` handles this for us. This is part of the FFI cost.
    let c_dir_name = CString::new(dir_name).expect("CString::new failed for dir_name");
    let c_content = CString::new(content).expect("CString::new failed for content");

    // --- Mark the end of preparation and start of the C call ---
    let preparation_duration = rust_start_time.elapsed();
    println!("Handing control over to C to create {} files...", file_count);

    // Calling C functions is `unsafe` because the Rust compiler cannot
    // guarantee the safety of the external code. We are taking responsibility here.
    let time_taken_by_c = unsafe {
        create_files_in_c(c_dir_name.as_ptr(), file_count as c_int, c_content.as_ptr())
    };

    // --- Mark the total time taken from Rust's perspective ---
    let total_rust_duration = rust_start_time.elapsed();

    // After the C function returns, we are back in safe Rust.
    if time_taken_by_c < 0.0 {
        println!("The C function reported an error.");
    } else {
        // Calculate the FFI overhead.
        // This is the total time measured in Rust minus the time the C code reported.
        // It accounts for string conversion, the call itself, and the return.
        let ffi_overhead = total_rust_duration.as_micros() as f64 / 1000.0 - time_taken_by_c;

        println!("\nRust has regained control!");
        println!("Successfully created {} files in the '{}' directory.", file_count, dir_name);
        println!("\n--- Timing Breakdown ---");
        println!("1. Rust data preparation:      {:.4}ms", preparation_duration.as_micros() as f64 / 1000.0);
        println!("2. C function execution:       {:.4}ms (as reported by C)", time_taken_by_c);
        println!("3. Total time (Rust's view):   {:.4}ms", total_rust_duration.as_micros() as f64 / 1000.0);
        println!("------------------------------------");
        // The overhead can sometimes be slightly negative due to timing precision differences,
        // so we'll ensure it's at least 0.
        println!("   FFI call overhead:          ~{:.4}ms (Total - C execution)", ffi_overhead.max(0.0));
    }
}
