use anyhow::{Context, Result};
use std::{collections::HashMap, path::{Path, PathBuf}};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct DirectoryAnalysis {
    pub total_size: u64,
    pub files: Vec<(PathBuf, u64)>,
    pub folders: HashMap<PathBuf, u64>,
    pub file_count: usize,
    pub dir_count: usize,
}

pub fn analyze_directory(path: &Path, show_hidden: bool) -> Result<DirectoryAnalysis> {
    let mut files = Vec::new();
    let mut folders = HashMap::new();
    let mut file_count = 0;
    let mut dir_count = 0;

    let walker = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            show_hidden || !is_hidden(e.path())
        });

    for entry in walker {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let entry_path = entry.path();
        
        if entry.file_type().is_file() {
            file_count += 1;
            let metadata = entry.metadata()
                .with_context(|| format!("Failed to read metadata for: {}", entry_path.display()))?;
            
            let size = metadata.len();
            files.push((entry_path.to_path_buf(), size));

            // Add file size to all parent directories
            let mut current = entry_path.parent();
            while let Some(parent) = current {
                folders
                    .entry(parent.to_path_buf())
                    .and_modify(|total| *total += size)
                    .or_insert(size);
                current = parent.parent();
            }
        } else if entry.file_type().is_dir() {
            dir_count += 1;
        }
    }

    // Sort files by size (largest first)
    files.sort_by(|a, b| b.1.cmp(&a.1));
    
    let total_size = files.iter().map(|(_, size)| *size).sum();

    Ok(DirectoryAnalysis {
        total_size,
        files,
        folders,
        file_count,
        dir_count,
    })
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, path::PathBuf};

    #[test]
    fn test_is_hidden() {
        assert!(is_hidden(Path::new(".hidden")));
        assert!(is_hidden(Path::new("/path/to/.hidden_file")));
        assert!(!is_hidden(Path::new("visible")));
        assert!(!is_hidden(Path::new("/path/to/visible_file")));
        assert!(!is_hidden(Path::new("file.txt")));
    }

    #[test]
    fn test_directory_analysis_structure() {
        let analysis = DirectoryAnalysis {
            total_size: 1024,
            files: vec![(PathBuf::from("test.txt"), 512)],
            folders: HashMap::new(),
            file_count: 1,
            dir_count: 1,
        };
        
        assert_eq!(analysis.total_size, 1024);
        assert_eq!(analysis.file_count, 1);
        assert_eq!(analysis.dir_count, 1);
        assert_eq!(analysis.files.len(), 1);
    }
}
