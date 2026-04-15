use crate::builder::ROUTESFILENAME;
use crate::server::file::File;
use crate::utils::macros::{exit_and_error, log_verbose};
use crate::utils::util;
use crate::{utils::manifest::FolderManifest};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]

/// RouteManifest contains the content type, size and file path of a route.
/// # Parameters
/// - [size]: The size of the file in bytes
/// - [file]: The file path of the file to be served for this route, for example "index.html"
pub struct RouteManifest {
    // pub content_type: String,
    pub size: u64,
    pub file: File,
    pub gzip: bool,
}
/// RouteT is a type alias for a HashMap where the key is the route path and the value is the RouteManifest which contains the content type, size and file path of the route.
/// # Parameters
/// - [String]: The route path, for example, "/index.html"
/// - [RouteManifest]: The manifest for the route which contains the content type, size and file path of the route.
pub type RouteT = HashMap<String, RouteManifest>;
/// Trinity is the return type of Routes::build,;it contains the new routes, the new files and the deleted files (if last_manifest is provided)
/// # Parameters
/// - [RouteT]: A HashMap of the new routes to be added to the server
/// -  [Vec<String>]: A list of new files that need to be extracted from the zip file and added to the server
/// -  [Option<Vec<String>>]: An optional list of files that need to be deleted
type Trinity = (RouteT, Vec<String>, Option<Vec<String>>);
#[derive(Debug, Clone, Serialize)]

pub struct Routes {
    pub routes: RouteT,
}
#[allow(dead_code)]
pub enum RouteExportKind {
    Json,
}

impl Routes {
    #[allow(dead_code)]
    pub fn export(&self, to: RouteExportKind, output_dir: &str) {
        match to {
            RouteExportKind::Json => {
                util::save_to_file(
                    serde_json::to_string(&self.routes)
                        .unwrap_or_else(|e| {
                            exit_and_error!("Failed to export routes: {}", e);
                        })
                        .as_bytes(),
                    format!("{}/{}.json", output_dir,ROUTESFILENAME).as_str(),
                );
            }
        }
    }
    pub fn get(&self, path: &str) -> Option<&RouteManifest> {
        let mut new_path = path.to_string();
        if !new_path.starts_with("/") {
            new_path = format!("/{}", new_path);
        }
        log_verbose!("Getting route info for : {}", &new_path);
        self.routes.get(new_path.as_str())
    }
    // fn get_mut(&mut self, path: &str) -> Option<&mut RouteManifest> {
    //     self.routes.get_mut(path)
    // }
    // fn insert(&mut self, path: String, route: RouteManifest) {
    //     self.routes.insert(path, route);
    // }
    // fn remove(&mut self, path: &str) -> Option<RouteManifest> {}
    fn make_file_route(file_info: &crate::utils::manifest::FileManifest) -> RouteManifest {
        RouteManifest {
            size: file_info.size,
            file: File::new(file_info.path.clone(),file_info.size),
            gzip: file_info.gzip,
        }
    }
    pub fn make_folder_route(folder_info: &FolderManifest, base: &str) -> (RouteT, Vec<File>) {
        let mut routes = HashMap::new();
        let mut files = Vec::new();
        for child in &folder_info.children {
            match child {
                crate::utils::manifest::PathType::File(file_info) => {
                    let route_info = Self::make_file_route(file_info);
                    files.push(route_info.file.clone());
                    // Remove the base path to create route, eg we can sever `/home/daniel/Projects/cask/temp/www/_app/immutable/chunks/DsnmJJEf.js` as what is requested is /_app/immutable/chunks/DsnmJJEf.js
                    let route = file_info.path.clone().replace(base, "");

                    routes.insert(route, route_info);
                }
                crate::utils::manifest::PathType::Folder(folder_info) => {
                    let (folder_routes, new_files) = Routes::make_folder_route(folder_info, base);
                    routes.extend(folder_routes);
                    files.extend(new_files);
                }
            }
        }
        (routes, files)
    }
    // fn get_routes(base: &str) -> RouteT {
    //     let manifest = crate::utils::::hash(base, base, false, "");
    //     let (routes, _) = Routes::make_folder_route(&manifest, base, base);
    //     routes
    // }
    pub fn build(manifest: &FolderManifest, last_manifest: Option<&FolderManifest>) -> Trinity {
        let (new_routes, new_files) = Routes::make_folder_route(manifest, manifest.path.as_str());
        match last_manifest {
            Some(last) => {
                let (last_routes, last_files) = Routes::make_folder_route(last, last.path.as_str());
                let mut files_to_delete = Vec::new();
                // compare last_files and new_files to find files to delete
                for file in last_files {
                    // TODO: Make more efficient
                    if new_files.iter().all(|x| x.path == file.path) {
                        files_to_delete.push(file.path);
                    }
                }
                // we add files newly added to the server and files that have been changed, we determine if a file has been changed by comparing the file size and file path of the new route and the last route, if either of them is different we consider the file to be changed and add it to the list of new files to be extracted from the zip file and added to the server
                let mut new_files = Vec::new();
                // compare new_routes and last_routes to find new files

                for (path, route) in &new_routes {
                    let last_route = last_routes.get(path);
                    match last_route {
                        Some(last_route) => {
                            // use file size and file name to determine if file should be overwritten
                            if last_route.size != route.size
                                || last_route.file.path != route.file.path
                            {
                                new_files.push(path.clone());
                            }
                        }
                        None => {
                            new_files.push(path.clone());
                        }
                    }
                }
                (new_routes, new_files, Some(files_to_delete))
            }
            None => {
                let new_files = new_routes.keys().cloned().collect();
                (new_routes, new_files, None)
            }
        }
    }
}
