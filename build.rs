use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the output directory for the build.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // *** UPDATED: Point to the new location of the Zig source file. ***
    let zig_file = "src/generator/generator.zig";
    let zig_dir = "src/generator/";

    // Tell Cargo to re-run this script if the Zig file or directory changes.
    println!("cargo:rerun-if-changed={}", zig_file);
    println!("cargo:rerun-if-changed={}", zig_dir);


    // Use the `zig build-lib` command to compile our Zig code into a static library.
    let status = Command::new("zig")
        .arg("build-lib")
        // Create Position-Independent Code for compatibility with Rust's default build.
        .arg("-fPIE")
        .arg("-fno-stack-protector") // Keep this for good measure
        .arg(zig_file)
        .arg(format!("-femit-bin={}/libgenerator.a", out_dir.display()))
        .arg("-O")
        .arg("ReleaseSmall") // This mode is less aggressive and better for libraries.
        .status()
        .expect("Failed to execute Zig build command");

    if !status.success() {
        panic!("Zig build failed");
    }

    // Tell Cargo where to find our newly compiled library.
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // Tell Cargo to link against our static library.
    println!("cargo:rustc-link-lib=static=generator");
}
