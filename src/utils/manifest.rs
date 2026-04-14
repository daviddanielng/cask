use crate::builder::MANIFESTFILENAME;
use crate::server;
use crate::utils::macros::{exit_and_error, log_verbose};
use crate::utils::util;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

static FILEEXTENSTIONTOGZIP: [&str; 10] = [
    "html", "css", "js", "json", "txt", "xml", "csv", "md", "svg", "ico",
];
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManifest {
    pub path: String,
    pub size: u64,
    pub hash: String,
    pub gzip: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PathType {
    File(FileManifest),
    Folder(FolderManifest),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderManifest {
    pub path: String,
    pub children: Vec<PathType>,
    pub size: u64,
}
pub(crate) struct Replace {
    pub(crate) to: String,
    pub(crate) replace: String,
}
pub fn get_file_manifest(path: &str, r: &Replace, gzip: bool, copy: bool) -> FileManifest {
    let new_path = path.replace(&r.replace, &r.to).to_string();

    let path_b = PathBuf::from(path);

    let hash = util::hash_file(path);
    if hash.is_err() {
        exit_and_error!("Failed to hash file {} :{}", path, hash.err().unwrap());
    }
    let mut file_manifest = FileManifest {
        path: new_path.clone(),
        size: 0,
        hash: hash.unwrap().to_string(),
        gzip: false,
    };
    let file_extension = path_b
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    if gzip && FILEEXTENSTIONTOGZIP.contains(&file_extension.as_str()) && copy {
        log_verbose!("zipping file {}", path);
        util::gzip_file(path, new_path.as_str());
        log_verbose!("file gzipped successfully");
        file_manifest.gzip = true;
        file_manifest.size = util::file_size(&new_path);
    } else {
        if copy {
            if !util::copy_file(&path, &new_path) {
                exit_and_error!("Failed to copy file {} to {}", path, new_path);
            }
            file_manifest.size = util::file_size(&new_path);
        } else {
            // if file was not copied, we just read the file size
            file_manifest.size = util::file_size(&path);
        }
    }
    file_manifest
}
pub fn get_folder_manifest(path: &str, r: &Replace, gzip: bool, copy: bool) -> FolderManifest {
    let new_path = path.replace(&r.replace, &r.to).to_string();
    let mut folder_manifest = FolderManifest {
        path: new_path,
        children: vec![],
        size: 0,
    };

    for entry in WalkDir::new(path).max_depth(1).contents_first(true) {
        let entry = entry.unwrap();
        let entry_path = entry.into_path();
        log_verbose!("Processing path {}", entry_path.display());

        let entry_path_str = entry_path.to_str().unwrap();
        let new_entry_path = entry_path_str.replace(&r.replace, &r.to).to_string();
        if entry_path.is_dir() {
            if copy {
                let create_dirs = util::create_dirs_not_existing(&new_entry_path);
                if !create_dirs {
                    exit_and_error!("Failed to create directory {}", new_entry_path);
                }
            }
            if entry_path_str == path {
                // to prevent a infinite loop, we skip start path
                continue;
            }

            let f_manifest = get_folder_manifest(entry_path_str, r, gzip, copy);
            // update folder size and children
            folder_manifest.size += f_manifest.size;
            folder_manifest.children.push(PathType::Folder(f_manifest));
        } else if entry_path.is_file() {
            let file_manifest = get_file_manifest(entry_path_str, r, gzip, copy);
            // update folder size and children

            folder_manifest.size += file_manifest.size;
            folder_manifest.children.push(PathType::File(file_manifest));
        } else if entry_path.is_symlink() {
            exit_and_error!("symlink file not supported {}", entry_path.display());
        } else {
            exit_and_error!("unkown file {}", entry_path.display());
        }
    }
    folder_manifest
}
pub fn get_manifest(start_path: &str, output: &str, gzip: bool, copy: bool) -> FolderManifest {
    if gzip && !copy {
        exit_and_error!("gzip is not supported when copy is false");
    }
    get_folder_manifest(
        start_path,
        &Replace {
            to: output.to_string(),
            replace: start_path.to_string(),
        },
        gzip,
        copy,
    )
}
pub fn extract_manifest_from_zip(zip: &File) -> FolderManifest {
    let zip_file = util::extract_from_zip(zip, crate::builder::MANIFESTFILENAME);
    match zip_file {
        Ok(c) => serde_json::from_slice(c.as_slice()).unwrap_or_else(|e| {
            exit_and_error!(
                "An error occurred while trying to parse manifest JSON data; Error: {}",
                e
            );
        }),
        Err(e) => {
            exit_and_error!(
                "An error occurred while trying to extract manifest from zip file; Error: {}",
                e
            )
        }
    }
}

pub fn get_last_manifest(output: &str) -> Option<FolderManifest> {
    let build_name = PathBuf::from(output).join(crate::builder::MANIFESTFILENAME);
    if !build_name.exists() {
        return None;
    }
    let file = File::open(build_name).unwrap_or_else(|e| {
        exit_and_error!(
            "An error occurred while trying to open manifest file; Error: {}",
            e
        );
    });
    Some(serde_json::from_reader(file).unwrap_or_else(|e| {
        exit_and_error!(
            "An error occurred while trying to parse manifest JSON data; Error: {}",
            e
        );
    }))
}

impl FolderManifest {
    pub fn replace_path(&mut self, teo: &str) {
        let mut new_to = teo.to_string();
        if new_to.ends_with("/") {
            // to prevent `/home/daniel/Projects/cask/temp/www//_app/immutable/assets`, we must remove the last `/` from to
            new_to.remove(new_to.len() - 1);
        }
        let old_path = self.path.clone();
        let mut new_children = vec![];
        self.path = new_to.clone();

        for child in self.children.clone() {
            match child {
                PathType::Folder(f) => {
                    let mut n = f.clone();
                    // build new path so if old_path is /home/daniel/ngi and current folder path is /home/daniel/ngi/_app, new_p becomes {to}/_app
                    let new_p = f.path.replace(&old_path, new_to.clone().as_str());
                    // temporary set n.path to old_path, so we can use replace_path to replace it with `to`, it will then be relace with new_path
                    n.path = old_path.clone();
                    n.replace_path(new_to.clone().as_str());
                    // set the correct path
                    n.path = new_p;
                    new_children.push(PathType::Folder(n))
                }
                PathType::File(f) => {
                    let mut n = f.clone();
                    n.path = f.path.replace(&old_path, new_to.clone().as_str());

                    new_children.push(PathType::File(n))
                }
            }
        }

        self.children = new_children;
    }

    pub fn save(&self, output: &str) -> String {
        if !util::is_dir(output) {
            exit_and_error!("{} is not a directory", output);
        }
        let new_path = Path::new(&output)
            .join(MANIFESTFILENAME)
            .to_string_lossy()
            .to_string();

        log_verbose!("Saving manifest to {}", new_path);

        util::save_to_file(
            serde_json::to_string(&self)
                .unwrap_or_else(|e| {
                    exit_and_error!("Failed to serialize folder hash data: {}", e);
                })
                .as_bytes(),
            new_path.as_str(),
        );
        new_path
    }
    pub fn files(&self) -> Vec<server::file::File> {
        let mut files = vec![];
        for child in self.children.clone() {
            match child {
                PathType::File(f) => {
                    files.push(server::file::File::new(f.path, f.size));
                }
                PathType::Folder(f) => {
                    files.extend(f.files());
                }
            }
        }
        files
    }
    pub fn files_to_map(&self) -> HashMap<String, server::file::File> {
        let mut files = HashMap::new();
        for child in self.children.clone() {
            match child {
                PathType::File(f) => {
                    files.insert(f.path.clone(), server::file::File::new(f.path, f.size));
                }
                PathType::Folder(f) => {
                    files.extend(f.files_to_map());
                }
            }
        }
        files
    }
}
