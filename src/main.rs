use std::fs;
use walkdir::WalkDir;

fn dir_size(path: &str) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| fs::metadata(e.path()).ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

fn format_bytes(size: u64) -> String {
    match size {
        s if s < 1024 => format!("{} B", s),
        s if s < 1024_u64.pow(2) => format!("{:.2} KB", s as f64 / 1024.0),
        s if s < 1024_u64.pow(3) => format!("{:.2} MB", s as f64 / 1024.0_f64.powi(2)),
        s if s < 1024_u64.pow(4) => format!("{:.2} GB", s as f64 / 1024.0_f64.powi(3)),
        s => format!("{:.2} TB", s as f64 / 1024.0_f64.powi(4)),
    }
}

fn main() {
    let path = "./";
    let size = dir_size(path);
    let dinfo = format_bytes(size);

    println!("Dir size is {dinfo}")
}
