// build.rs

fn main() {
    // This tells the `cc` crate to find our C source file.
    cc::Build::new()
        .file("c_src/file_creator.c")
        .compile("file_creator_lib"); // The output will be `libfile_creator_lib.a`

    // This tells Cargo to re-run the build script if our C code changes.
    println!("cargo:rerun-if-changed=c_src/file_creator.c");
    println!("cargo:rerun-if-changed=c_src/file_creator.h");
}
