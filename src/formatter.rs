use crate::analyzer::DirectoryAnalysis;
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

pub fn format_bytes(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;
    
    if size == 0 {
        return "0 B".to_string();
    }
    
    let size_f = size as f64;
    let unit_index = (size_f.log2() / THRESHOLD.log2()).floor() as usize;
    let unit_index = unit_index.min(UNITS.len() - 1);
    
    let value = size_f / THRESHOLD.powi(unit_index as i32);
    
    if unit_index == 0 {
        format!("{} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", value, UNITS[unit_index])
    }
}

pub fn display_results(analysis: &DirectoryAnalysis, top_files: usize, top_dirs: usize, show_all: bool, no_color: bool) -> Result<()> {
    let use_color = !no_color && colored::control::SHOULD_COLORIZE.should_colorize();
    
    // Summary header
    let header = "📁 Directory Analysis Summary";
    
    if use_color {
        println!("{}", header.bright_cyan().bold());
    } else {
        println!("{}", header);
    }
    
    println!("{}", "=".repeat(50));
    
    // Summary statistics
    let stats = vec![
        ("Total Size", format_bytes(analysis.total_size)),
        ("Files", analysis.file_count.to_string()),
        ("Directories", analysis.dir_count.to_string()),
    ];
    
    for (label, value) in stats {
        if use_color {
            println!("{:12}: {}", label.bright_white().bold(), value.bright_green());
        } else {
            println!("{:12}: {}", label, value);
        }
    }
    
    println!();

    // Top files
    let file_limit = if show_all { analysis.files.len() } else { top_files };
    let top_files = analysis.files.iter().take(file_limit);
    
    let files_header = format!("📄 Top {} Largest Files", file_limit.min(analysis.files.len()));
    if use_color {
        println!("{}", files_header.bright_yellow().bold());
    } else {
        println!("{}", files_header);
    }
    println!("{}", "-".repeat(30));
    
    for (i, (path, size)) in top_files.enumerate() {
        let size_str = format_bytes(*size);
        let path_str = path.display().to_string();
        
        if use_color {
            println!(
                "{:3}. {:>10} {}",
                (i + 1).to_string().bright_blue(),
                size_str.bright_green(),
                path_str.normal()
            );
        } else {
            println!("{:3}. {:>10} {}", i + 1, size_str, path_str);
        }
    }
    
    println!();

    // Top folders
    let mut folders_vec: Vec<(PathBuf, u64)> = analysis.folders.iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    
    folders_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    let folder_limit = if show_all { folders_vec.len() } else { top_dirs };
    let top_folders = folders_vec.iter().take(folder_limit);
    
    let folders_header = format!("📂 Top {} Largest Directories", folder_limit.min(folders_vec.len()));
    if use_color {
        println!("{}", folders_header.bright_magenta().bold());
    } else {
        println!("{}", folders_header);
    }
    println!("{}", "-".repeat(30));
    
    for (i, (path, size)) in top_folders.enumerate() {
        let size_str = format_bytes(*size);
        let path_str = path.display().to_string();
        
        if use_color {
            println!(
                "{:3}. {:>10} {}",
                (i + 1).to_string().bright_blue(),
                size_str.bright_green(),
                path_str.normal()
            );
        } else {
            println!("{:3}. {:>10} {}", i + 1, size_str, path_str);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024_u64.pow(3)), "1.00 GB");
        assert_eq!(format_bytes(1024_u64.pow(4)), "1.00 TB");
        assert_eq!(format_bytes(1024_u64.pow(5)), "1.00 PB");
    }

    #[test]
    fn test_format_bytes_edge_cases() {
        // Test exact powers of 1024
        assert_eq!(format_bytes(1024_u64.pow(2)), "1.00 MB");
        assert_eq!(format_bytes(2 * 1024_u64.pow(2)), "2.00 MB");
    }
}
