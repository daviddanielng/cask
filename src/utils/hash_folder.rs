//! Utilities for recursively collecting metadata about a directory tree.
//!
//! This module walks a directory, gathers file and folder size information,
//! and builds a serializable tree structure describing its contents.
static FILEEXTENSTIONTOGZIP: [&str; 10] = [
    "html", "css", "js", "json", "txt", "xml", "csv", "md", "svg", "ico",
];
use std::path::PathBuf;

use crate::utils::{
    logger::{log_verbose, log_warning},
    util::{self, exit_with_error},
};
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    path: String,
    size: u64,
    hash: String,
    gzip: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    File(FileInfo),
    Folder(FolderInfo),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    path: String,
    children: Vec<PathType>,
    size: u64,
}

/// Metadata collected for a single file.
///
/// The `hash` field stores an XXH3 64-bit hash derived from the file path
/// string, not from the file contents.

/// Represents either a file or a folder node in the directory tree.

/// Metadata collected for a folder.
///
/// `children` contains all nested files and subfolders discovered during
/// traversal, and `size` is the cumulative size of all contained files.

/// Recursively traverses `path` and returns a [`FolderInfo`] describing its contents.
///
/// # Parameters
/// - `path`: The directory to scan.
/// - `main_path`: The root path prefix removed from discovered entry paths,
///   allowing stored file paths to be relative to the main folder.
///
/// # Behavior
/// - Verifies that `path` exists and is a directory.
/// - Recursively visits subdirectories.
/// - Collects file sizes and a path-based XXH3 hash for each file.
/// - Skips symbolic links and unknown path types, logging a warning for each.
///
/// # Returns
/// A [`FolderInfo`] containing the discovered children and the total size of
/// all files found under `path`.  `very stupid, i know but the is most straightforward way to get a relative path without adding another dependency just for that`
///
/// # Panics / Termination
/// This function does not return recoverable errors. It terminates the program
/// via `exit_with_error` if the directory does not exist, is not a directory,
/// or cannot be read.
pub fn hash(path: &str, main_path: &str, gzip: bool, save_to: &str) -> FolderInfo {
    if !util::path_exists(path) {
        exit_with_error(format!("{} does not exist", path).as_str())
    }
    if !util::is_dir(path) {
        exit_with_error(format!("{} is not a dir", path).as_str())
    }
    let mut folder_info = FolderInfo {
        path: save_to.to_string(),
        children: Vec::new(),
        size: 0,
    };

    let read_dir = std::fs::read_dir(path).unwrap_or_else(|e| {
        exit_with_error(format!("Failed to read directory {}: {}", path, e).as_str())
    });
    for entry in read_dir {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                let entry_path_str = entry_path.to_str().unwrap_or("");
                let relative_path = entry_path.to_string_lossy().to_string().replace(path, "");
                let out_path = PathBuf::from(&save_to)
                    .join(&relative_path.trim_start_matches(std::path::MAIN_SEPARATOR));
                let out_path_str = out_path.to_str().unwrap_or("");
                log_verbose(format!("Processing file: {}", &entry_path_str).as_str());
                if entry_path.is_file() {
                    log_verbose("getting file info");
                    let metadata = entry.metadata().unwrap_or_else(|e| {
                        exit_with_error(
                            format!("Failed to get metadata for file {}: {}", relative_path, e)
                                .as_str(),
                        )
                    });
                    let file_size = metadata.len();
                    let file_extension = entry_path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    let mut file_info = FileInfo {
                        path: relative_path,
                        size: file_size,
                        hash: xxh3_64(entry_path_str.as_bytes()).to_string(),
                        gzip: false,
                    };
                    if gzip && FILEEXTENSTIONTOGZIP.contains(&file_extension.as_str()) {
                        log_verbose("gzipping file");
                        util::gzip_file(&entry_path_str, &out_path_str);
                        log_verbose("file gzipped successfully, updating folder info");
                        file_info.gzip = true;
                    } else {
                        log_verbose("copying file to temp directory");
                        if !util::copy_file(&entry_path_str, &out_path_str) {
                            exit_with_error(
                                format!(
                                    "Failed to copy file {} to {}",
                                    entry_path_str, out_path_str
                                )
                                .as_str(),
                            );
                        }
                        log_verbose("file copied successfully, updating folder info");
                    }
                    folder_info.size += file_size;
                    folder_info.children.push(PathType::File(file_info));
                } else if entry_path.is_dir() {
                    if !out_path.exists() {
                        if !util::create_dirs(&out_path_str) {
                            exit_with_error(
                                format!(
                                    "Failed to create directory for hashing at {}",
                                    out_path_str
                                )
                                .as_str(),
                            );
                        }
                    }
                    let children = hash(&entry_path_str, main_path, gzip, out_path_str);

                    folder_info.size += children.size;
                    folder_info.children.push(PathType::Folder(children));
                    // folder_info.path = relative_path.to_string();
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
