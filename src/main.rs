mod utils;
mod analyzer;

use utils::format_bytes;
use analyzer::{scan_files, get_top_files, get_top_folders};
use colored::Colorize;

fn main() {
    let path = "./";

    // Scan the filesystem
    let (files, folders, total_size) = scan_files(path);
    println!("{}", "=== Summary ===".bold().blue());
    println!(
        "Total size : {}",
        format_bytes(total_size).to_string().red().bold()
    );

    // Top 5 files
    let top_files = get_top_files(&files, 5);
    println!("\n{}", "== Top 5 Files ==".bold().blue());
    for (i, (size, path)) in top_files.iter().enumerate() {
        println!(
            "  {:>2}. {:>10}  {}",
            i + 1,
            format_bytes(*size).yellow(),
            path.display().to_string().green(),
        );
    }

    // Top 5 folders
    let top_folders = get_top_folders(&folders, 5);
    println!("\n{}", "== Top 5 Folders ==".bold().blue());
    for (i, (size, path)) in top_folders.iter().enumerate() {
        println!(
            "  {:>2}. {:>10}  {}",
            i + 1,
            format_bytes(*size).yellow(),
            path.display().to_string().green(),
        );
    }
}
