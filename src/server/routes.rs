use std::collections::HashMap;

use crate::{server::routes, utils::hash_folder::FolderManifest};

#[derive(Debug, Clone)]

/// RouteManifest contains the content type, size and file path of a route.
/// # Parameters
/// -  [content_type]: The content type of the route, for example "text/html'
/// -  [size]: The size of the file in bytes
/// -  [file]: The file path of the file to be served for this route, for example "index.html"
pub struct RouteManifest {
    pub content_type: String,
    pub size: u64,
    pub file: String,
}
/// RouteT is a type alias for a HashMap where the key is the route path and the value is the RouteManifest which contains the content type, size and file path of the route.
/// # Parameters
/// -  [String]: The route path, for example "/index.html"
/// -  [RouteManifest]: The manifest for the route which contains the content type, size and file path of the route.
type RouteT = HashMap<String, RouteManifest>;
/// Trinity is the return type of Routes::build, it contains the new routes, the new files and the deleted files (if last_manifest is provided)
/// # Parameters
/// -  [RouteT]: A HashMap of the new routes to be added to the server
/// -  [Vec<String>]: A list of new files that need to be extracted from the zip file and added to the server
/// -  [Option<Vec<String>>]: An optional list of files that need to be deleted
type Trinity = (RouteT, Vec<String>, Option<Vec<String>>);
#[derive(Debug, Clone)]

pub struct Routes {
    pub routes: RouteT,
}
impl Routes {
    fn make_file_route(
        file_info: &crate::utils::hash_folder::FileInfo,
        folder_path: &str,
    ) -> RouteManifest {
        let file_path = format!("{}{}", folder_path, file_info.path);
        RouteManifest {
            content_type: "application/octet-stream".to_string(),
            size: file_info.size,
            file: file_path,
        }
    }
    fn make_folder_route(
        folder_info: &crate::utils::hash_folder::FolderManifest,
        replace: &str,
    ) -> (RouteT, Vec<String>) {
        let mut routes = HashMap::new();
        let start_path = folder_info.path.clone().replace(replace, "");
        let mut files = Vec::new();
        for child in &folder_info.children {
            match child {
                crate::utils::hash_folder::PathType::File(file_info) => {
                    let route_path = format!("{}{}", start_path, file_info.path);
                    let route_info = Self::make_file_route(file_info, replace);
                    files.push(route_info.file.clone());
                    routes.insert(route_path, route_info);
                }
                crate::utils::hash_folder::PathType::Folder(folder_info) => {
                    let (folder_routes, new_files) =
                        Routes::make_folder_route(folder_info, replace);
                    routes.extend(folder_routes);
                    files.extend(new_files);
                }
            }
        }
        (routes, files)
    }
    pub fn build(manifest: &FolderManifest, last_manifest: Option<&FolderManifest>) -> Trinity {
        let (new_routes, new_files) = Routes::make_folder_route(manifest, &manifest.path);
        match last_manifest {
            Some(last) => {
                let (last_routes, last_files) = Routes::make_folder_route(last, &last.path);
                let mut files_to_delete = Vec::new();
                // compare last_files and new_files to find files to delete
                for file in last_files {
                    if !new_files.contains(&file) {
                        files_to_delete.push(file);
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
                            if last_route.size != route.size || last_route.file != route.file {
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
