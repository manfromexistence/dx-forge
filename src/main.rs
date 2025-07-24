use std::fs::{create_dir, OpenOptions};
use std::io::{BufWriter, Write};
use std::time::Instant;
use rayon::prelude::*;

fn precreate_files(paths: &[String]) {
    paths.par_iter().for_each(|path| {
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Failed to precreate file");
    });
}

fn write_files(paths: &[String], data: &[u8]) {
    paths.par_iter().for_each(|path| {
        let file = OpenOptions::new()
            .write(true)
            .open(path)
            .expect("Failed to open file");
        let mut writer = BufWriter::with_capacity(64, file);
        writer.write_all(data).expect("Failed to write to file");
    });
}

fn main() {
    let start = Instant::now();
    create_dir("rust_modules").unwrap_or(());
    let data = b"Hello, manfromexistence";
    let paths: Vec<String> = (0..1000)
        .map(|i| format!("rust_modules/f{}.txt", i))
        .collect();
    precreate_files(&paths);
    write_files(&paths, data);
    let duration = start.elapsed();
    let time_ms = duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0;
    println!("Time taken: {:.3} ms", time_ms);
}