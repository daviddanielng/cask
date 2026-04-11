pub mod config;
pub mod engine;
pub mod routes;
pub mod file;

use crate::server::routes::RouteT;
use crate::{
    server::routes::Routes,
    utils::{
        executable,
        manifest::{extract_manifest_from_zip, get_last_manifest},
        macros::{exit_and_error, log_verbose},
        util,
    },
};
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use crate::utils::macros::log_info;

pub fn start_dev_serve() {}
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
    let routes = extract_files(&zip_file, &config.output);
    actix_web::rt::System::new()
        .block_on(engine::start(config.port, Routes { routes }))
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to start server on port {}: {}", config.port, e);
        });


    // log_info!("home is {:?}", manifest);
    //     let mut exe = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    //     exe.seek(SeekFrom::End(-16)).unwrap();
    //     let mut tail = [0u8; 16];
    //     exe.read_exact(&mut tail).unwrap();
    //     assert_eq!(&tail[8..], b"SFS12345");
    //     let file_size = u64::from_le_bytes(tail[0..8].try_into().unwrap());
    //     // Seek to where file bytes start
    //     exe.seek(SeekFrom::End(-(16 + file_size as i64))).unwrap();

    //     let mut file_bytes = vec![0u8; file_size as usize];
    //     exe.read_exact(&mut file_bytes).unwrap();

    //     println!("{}", String::from_utf8(file_bytes).unwrap());
}
fn extract_files(zip_file: &File, output: &str) -> RouteT {
    let mut new_manifest = extract_manifest_from_zip(zip_file);
    // replace the creation path with new path, so if you built in program with input /home/daniel/projects/www/here, it would be replaced with `output`, this makes it easy to read file after extracting
    new_manifest.replace_path(output);
    let last_manifest = get_last_manifest(output);
    let (routes, new_files, deleting) =
        Routes::build(&new_manifest, last_manifest.as_ref());
    check_del(deleting);
    extract_new(zip_file, new_files,output);
    new_manifest.save( output);
    routes
}


fn extract_new(zip: &File, files: Vec<String>,base:&str) {
    if files.len()==0{
        log_info!("No new files");
        return;
    }

    for file in files {
        // removing base path because we are extracting file from zip, so /home/daniel/ddd/app.css becomes /ddd/app.css
        let mut new_file = file.replace(base,"").clone();
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
