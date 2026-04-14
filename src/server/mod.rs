pub mod config;
pub mod dev_serve;
pub mod engine;
pub mod file;
pub mod routes;

use crate::server::routes::RouteT;
use crate::utils::macros::log_info;
use crate::{
    server::routes::Routes,
    utils::{
        executable,
        macros::{exit_and_error, log_verbose},
        manifest::{extract_manifest_from_zip, get_last_manifest},
        util,
    },
};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

pub fn start_server(config: crate::args::server::ServerRunConfig) {
    let (zip_file, zip_file_path) = executable::read_files(&config);

    log_verbose!("Reading zip manifest from extracted embedded zip file.");
    match &config.fallback {
        Some(file) => {
            log_verbose!(
                "Fallback file specified in config: {}. This file will be returned for any missing files.",
                file
            );
            let file_in_zip = util::file_exists_in_zip(&zip_file_path, &file);
            if !file_in_zip {
                exit_and_error!(
                    "Fallback file specified in config ({}) does not exist in embedded zip file.",
                    file
                );
            } else {
                log_verbose!("Fallback file exists in zip file.");
            }
        }
        None => log_verbose!(
            "No fallback file specified in config, 404 will be returned for missing files."
        ),
    }
    let new_routes = extract_files(&zip_file, &config.output);
    let shared = Arc::new(tokio::sync::RwLock::new(new_routes));
    let bg_shared = shared.clone();
    actix_web::rt::System::new()
        .block_on(engine::start(config.port, bg_shared, config.fallback))
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to start server on port {}: {}", config.port, e);
        });
}
fn extract_files(zip_file: &File, output: &str) -> RouteT {
    let mut new_manifest = extract_manifest_from_zip(zip_file);
    // replace the creation path with new path, so if you built in program with input /home/daniel/projects/www/here, it would be replaced with `output`, this makes it easy to read file after extracting
    new_manifest.replace_path(output);
    let last_manifest = get_last_manifest(output);
    let (routes, new_files, deleting) = Routes::build(&new_manifest, last_manifest.as_ref());
    check_del(deleting);
    extract_new(zip_file, new_files, output);
    new_manifest.save(output);
    routes
}

fn extract_new(zip: &File, files: Vec<String>, base: &str) {
    if files.len() == 0 {
        log_info!("No new files");
        return;
    }

    for file in files {
        // removing base path because we are extracting file from zip, so /home/daniel/ddd/app.css becomes /ddd/app.css
        let mut new_file = file.replace(base, "").clone();
        if new_file.starts_with("/") {
            // we have the zip file, / is the start of root path which is not what we want.
            new_file.remove(0);
        }
        // file is already been set with [!FolderManifest.replace_path()], just save it the file, modifying here means route won't find the file.
        let new_path = PathBuf::from(&file);
        log_verbose!("Extracting {} to {:?}", new_file, new_path);

        let z_file = util::extract_from_zip(&zip, &new_file);
        match z_file {
            Ok(c) => {
                if !new_path.parent().unwrap().exists() {
                    fs::create_dir_all(new_path.parent().unwrap()).unwrap_or_else(|err| {
                        exit_and_error!(
                            "Failed to create directory {}: {}",
                            new_path.parent().unwrap().display(),
                            err
                        );
                    });
                }
                fs::write(&new_path, c).unwrap_or_else(|err| {
                    exit_and_error!("Failed to write file {}: {}", new_path.display(), err);
                });
            }
            Err(e) => {
                exit_and_error!("Failed to extract file {} from zip: {}", new_file, e);
            }
        }
    }
}

fn check_del(files: Option<Vec<String>>) {
    if let Some(files) = files {
        for file in files {
            log_verbose!("Deleting file: {}", file);
            util::delete_file(file.as_str());
        }
    }
}
