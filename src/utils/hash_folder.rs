use crate::utils::{
    logger::log_warning,
    util::{self, exit_with_error},
};
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    size: u64,
    hash: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    File(FileInfo),
    Folder(FolderInfo),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    children: Vec<PathType>,
    size: u64,
}

pub fn hash(path: &str) -> Vec<PathType> {
    if !util::path_exists(path) {
        exit_with_error(format!("{} does not exist", path).as_str())
    }
    if !util::is_dir(path) {
        exit_with_error(format!("{} is not a dir", path).as_str())
    }
    let mut folder_info = Vec::<PathType>::new();

    let read_dir = std::fs::read_dir(path).unwrap_or_else(|e| {
        exit_with_error(format!("Failed to read directory {}: {}", path, e).as_str())
    });

    for entry in read_dir {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    let metadata = entry.metadata().unwrap_or_else(|e| {
                        exit_with_error(
                            format!(
                                "Failed to get metadata for file {}: {}",
                                entry_path.to_str().unwrap_or("Invalid UTF-8 path"),
                                e
                            )
                            .as_str(),
                        )
                    });
                    folder_info.push(PathType::File(FileInfo {
                        path: entry_path.to_str().unwrap_or("").to_string(),
                        size: metadata.len(),
                        hash: xxh3_64(entry_path.to_str().unwrap_or("").as_bytes()).to_string(),
                    }));
                } else if entry_path.is_dir() {
                    let children = hash(entry_path.to_str().unwrap_or(""));
                    let size = children
                        .iter()
                        .map(|child| match child {
                            PathType::File(file_info) => file_info.size,
                            PathType::Folder(folder_info) => folder_info.size,
                        })
                        .sum();
                    folder_info.push(PathType::Folder(FolderInfo { children, size }));
                } else if entry_path.is_symlink() {
                    log_warning(
                        format!(
                            "Skipping symbolic link: {}",
                            entry_path.to_str().unwrap_or("Invalid UTF-8 path")
                        )
                        .as_str(),
                    );
                } else {
                    log_warning(
                        format!(
                            "Skipping unknown path type: {}",
                            entry_path.to_str().unwrap_or("Invalid UTF-8 path")
                        )
                        .as_str(),
                    );
                }
            }
            Err(e) => {
                exit_with_error(format!("Failed to read directory {}: {}", path, e).as_str());
            }
        }
    }

    folder_info
}
