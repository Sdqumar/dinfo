use std::path::PathBuf;
use std::collections::HashMap;

pub fn add_to_parents(folders: &mut HashMap<PathBuf, u64>, path: &PathBuf, size: u64) {
    let mut current = path.parent();
    while let Some(parent) = current {
        if !parent.as_os_str().is_empty() {
        folders
            .entry(parent.to_path_buf())
            .and_modify(|total| *total += size)
            .or_insert(size);
        }
        current = parent.parent();
    }
}

pub fn format_bytes(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: f64 = 1024.0;
    let mut size = size as f64;
    let mut unit = 0;
    while size >= THRESHOLD && unit < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit += 1;
    }
    if size.fract() == 0.0 {
        format!("{:.0} {}", size, UNITS[unit])
    } else {
        format!("{:.2} {}", size, UNITS[unit])
    }
}
