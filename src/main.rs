use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

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
    // Define the parameters for our C function in Rust.
    let dir_name = "c_modules";
    let file_count = 100;
    let content = "Hello, manfromexistence!";

    println!("Handing control over to C to create {} files...", file_count);

    // To pass strings to C, they must be null-terminated.
    // `CString` handles this for us.
    let c_dir_name = CString::new(dir_name).expect("CString::new failed for dir_name");
    let c_content = CString::new(content).expect("CString::new failed for content");

    // Calling C functions is `unsafe` because the Rust compiler cannot
    // guarantee the safety of the external code. We are taking responsibility here.
    let time_taken = unsafe {
        create_files_in_c(c_dir_name.as_ptr(), file_count as c_int, c_content.as_ptr())
    };

    // After the C function returns, we are back in safe Rust.
    if time_taken < 0.0 {
        println!("The C function reported an error.");
    } else {
        println!("\nRust has regained control!");
        println!("Successfully created {} files in the '{}' directory.", file_count, dir_name);
        println!("C code reported that it took {:.4}ms to complete.", time_taken);
    }
}
