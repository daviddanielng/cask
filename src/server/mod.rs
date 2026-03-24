pub mod config;
pub mod engine;
pub mod routes;

use std::fs::File;

use crate::{
    server::routes::Routes,
    utils::{
        executable,
        hash_folder::{extract_manifest_from_zip, get_last_manifest},
        macros::{exit_and_error, log_verbose},
        util,
    },
};

pub fn start_server(config: crate::server::config::config::ServerRunConfig) {
    let (mut zip_file, zip_file_path) = executable::read_files(&config);
    log_verbose!("Reading zip manifest from extracted embedded zip file.");
    match config.fallback {
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
    extract_files(zip_file, &config.output);
    // let manifest = extract_manifest_from_zip(&mut zip_file);
    actix_web::rt::System::new()
        .block_on(engine::start(config.port))
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
fn extract_files(mut zip_file: File, output: &str) {
    let new_manifest = extract_manifest_from_zip(&mut zip_file);
    let last_manifest = get_last_manifest(output);
    let new_routes = Routes::build(&new_manifest, last_manifest.as_ref(), output);
    // dbg!(new_routes);
    // match last_manifest {
    //     Some(manifest) => {
    //         let last_routes = Routes::new(manifest);
    //         new_routes.compare_and_extract(&mut zip_file, &last_routes, output);
    //     }
    //     None => {
    //         log_verbose!("No previous manifest found, extracting all files from zip.");
    //         // new_routes.extract_all(&mut zip_file, output);
    //     }
    // }
}
