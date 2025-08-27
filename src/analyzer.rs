use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils::add_to_parents;

pub fn scan_files(path: &str) -> (Vec<(PathBuf, u64)>, HashMap<PathBuf, u64>, u64) {
    let mut files: Vec<(PathBuf, u64)> = Vec::new();
    let mut folders: HashMap<PathBuf, u64> = HashMap::new();

    let entries = WalkDir::new(path).into_iter().filter_map(|e| e.ok());

    for entry in entries {
        let entry_path = entry.path();
        if entry.file_type().is_file() {
            if let Ok(metadata) = entry.metadata() {
                let size = metadata.len();
                let path_buf = entry_path.to_path_buf();
                files.push((path_buf.clone(), size));
                add_to_parents(&mut folders, &path_buf, size);
            }
        }
    }

    let total_size: u64 = files.iter().map(|(_, size)| *size).sum();
    (files, folders, total_size)
}

pub fn get_top_files(files: &[(PathBuf, u64)], top_n: usize) -> Vec<(u64, PathBuf)> {
    let mut file_heap: BinaryHeap<Reverse<(u64, PathBuf)>> = BinaryHeap::new();
    for (path, size) in files {
        file_heap.push(Reverse((*size, path.clone())));
        if file_heap.len() > top_n {
            file_heap.pop();
        }
    }
    file_heap
        .into_sorted_vec()
        .into_iter()
        .map(|Reverse((s, p))| (s, p))
        .collect()
}

pub fn get_top_folders(folders: &HashMap<PathBuf, u64>, top_n: usize) -> Vec<(u64, PathBuf)> {
    let mut folder_heap: BinaryHeap<Reverse<(u64, PathBuf)>> = BinaryHeap::new();
    for (path, size) in folders {
        folder_heap.push(Reverse((*size, path.clone())));
        if folder_heap.len() > top_n {
            folder_heap.pop();
        }
    }
    folder_heap
        .into_sorted_vec()
        .into_iter()
        .map(|Reverse((s, p))| (s, p))
        .collect()
}
