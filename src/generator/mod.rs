//! # Generator Module
//!
//! This module acts as a bridge to the high-performance Zig code.
//! It provides a safe Rust interface for the FFI (Foreign Function Interface)
//! calls, encapsulating the `unsafe` logic required to interact with the
//! compiled Zig library.

// This block tells Rust about the function we exported from our Zig library.
#[link(name = "generator", kind = "static")]
extern "C" {
    /// This function is defined in `src/generator/generator.zig` and compiled by `build.rs`.
    fn process_tsx_file(path_ptr: *const u8, path_len: usize) -> bool;
}

/// A safe Rust wrapper around the unsafe FFI call to the Zig generator.
pub fn process_file(path_str: &str) -> bool {
    // Calling an external C function is `unsafe` because Rust's
    // compiler cannot guarantee its memory safety. We trust our Zig code.
    unsafe {
        process_tsx_file(path_str.as_ptr(), path_str.len())
    }
}