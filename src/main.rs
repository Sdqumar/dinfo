use std::{collections::HashMap, fs, path::PathBuf};
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

    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    let mut folders: HashMap<PathBuf, u64> = HashMap::new();

    let entries = WalkDir::new(path).into_iter().filter_map(|e| e.ok());

    for entry in entries {
        let entry_path = entry.path();
        if entry.file_type().is_file() {
            let metadata = entry.metadata().ok().unwrap();
            let size = metadata.len();
            files.push((entry_path.to_path_buf(), size));

            let mut current = entry_path.parent();
            while let Some(parent) = current {
                folders
                    .entry(parent.to_path_buf())
                    .and_modify(|total| *total += size)
                    .or_insert(size);
                current = parent.parent();
            }
        }
    }

    // println!("{:#?}", files);
    // println!("{:#?}", folders);
    // let size = dir_size(path);
    // let dinfo = format_bytes(size);

    // println!("Dir size is {dinfo}")

    files.sort_by(|a, b| b.1.cmp(&a.1));

    let total_size: u64 = files.iter().map(|(_, size)| *size).sum();
    println!("Total size of all files: {}", format_bytes(total_size));
    // Take top 5
    let top_files = files.iter().take(5);

    println!("Top 5 Files:");
    for (path, size) in top_files {
        println!("  {:?}  {}", path, format_bytes(*size));
    }

    let mut folders_vec: Vec<(PathBuf, u64)> = folders.into_iter().collect();

    // Sort by size (largest first)
    folders_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top 5
    let top_folders = folders_vec.iter().take(5);

    println!("\nTop 5 Folders:");
    for (path, size) in top_folders {
        println!("  {:?}  {}", path, format_bytes(*size));
    }
}
