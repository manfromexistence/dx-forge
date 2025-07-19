use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let zig_file = "src/generator/generator.zig";
    let zig_dir = "src/generator/";

    println!("cargo:rerun-if-changed={}", zig_file);
    println!("cargo:rerun-if-changed={}", zig_dir);

    let status = Command::new("zig")
        .arg("build-lib")
        .arg("-fPIE")
        .arg("-fno-stack-protector")
        .arg(zig_file)
        .arg(format!("-femit-bin={}/libgenerator.a", out_dir.display()))
        .arg("-O")
        .arg("ReleaseSmall")
        .status()
        .expect("Failed to execute Zig build command");

    if !status.success() {
        panic!("Zig build failed");
    }

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=generator");
}