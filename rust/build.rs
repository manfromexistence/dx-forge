use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the output directory for the build.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Define the path to our Zig source file.
    let zig_file = "src/generator.zig";

    // Tell Cargo to re-run this script if the Zig file changes.
    println!("cargo:rerun-if-changed={}", zig_file);

    // Use the `zig build-lib` command to compile our Zig code into a static library.
    // We place the output library in the build script's output directory.
    let status = Command::new("zig")
        .arg("build-lib")
        .arg(zig_file)
        .arg(format!("-femit-bin={}/libgenerator.a", out_dir.display()))
        .arg("-fallow-shaking") // Allow unused symbols to be removed
        .arg("-O") // Optimize for speed
        .arg("ReleaseSafe")
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
