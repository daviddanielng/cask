use std::collections::HashMap;

use crate::utils::{executable::build, hash_folder::FolderManifest};

pub struct RouteManifest {
    pub path: String,
    pub content_type: String,
    pub size: u64,
}
pub struct Routes {
    pub routes: HashMap<String, RouteManifest>,
}
impl Routes {
    pub fn new(manifest: FolderManifest) -> Self {
        Routes::build(manifest);
        Self {
            routes: HashMap::new(),
        }
    }
    fn build(manifest: FolderManifest) {
        for child in manifest.children {
            match child {
                crate::utils::hash_folder::PathType::File(file_info) => {
                    println!("File: {}, size: {}", file_info.path, file_info.size);
                }
                crate::utils::hash_folder::PathType::Folder(folder_info) => {
                    Routes::build(folder_info.clone());
                    println!("Folder: {}, size: {}", folder_info.path, folder_info.size);
                }
            }
        }
    }
    pub fn add_route(&mut self, path: String, content_type: String) {}
}
