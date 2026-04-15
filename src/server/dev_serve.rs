use crate::builder::MANIFESTFILENAME;
use crate::server::routes::{RouteT, Routes};
use crate::utils::macros::{exit_and_error, log_verbose,log_info};
use crate::utils::manifest::FolderManifest;
use std::path::PathBuf;
use std::sync::Arc;

pub fn start_dev_serve(input: PathBuf, port: u16) {
    let new_routes =first_load_routes(input.to_str().unwrap());
    let shared = Arc::new(tokio::sync::RwLock::new(new_routes));
    let bg_shared = shared.clone();

    actix_web::rt::System::new()
        .block_on(async move {
            // Reload manifest every 5 seconds  
            actix_web::rt::spawn(async move {
                let spawn_shared = shared.clone();
                let new_input_str = input.to_str().unwrap();
                let mut interval = actix_web::rt::time::interval(std::time::Duration::from_secs(5));
                loop {
                    interval.tick().await;

                    let new_manifest = load_current_manifest(new_input_str);
                    let changes = see_changes(&new_manifest, new_input_str);
                    let count_changes = changes.len();
                    if count_changes > 0 {
                        let mut m = spawn_shared.write().await;
                        let (new_routes, _) =
                            Routes::make_folder_route(&new_manifest, new_manifest.path.as_str());
                        *m = new_routes;

                        log_info!("route reloaded! ({} changes)", count_changes);
                    }
                }
            });

            crate::server::engine::start(port, bg_shared, None).await
        })
        .unwrap_or_else(|e| {
            exit_and_error!("Failed to start server on port {}: {}", port, e);
        });
}
fn load_current_manifest(input:&str) -> FolderManifest{
    crate::utils::manifest::get_manifest(
        input,
        input,
        false,
        false,
    )
}
fn first_load_routes(input:&str) -> RouteT{
  let manifest=load_current_manifest(input);
    Routes::make_folder_route(&manifest, input).0
}
fn see_changes(new_manifest: &FolderManifest, watch_dir: &str) -> Vec<String> {
    log_verbose!("Checking for changes in manifest files in {} ", watch_dir);
    let mut changes = vec![];
    let last_manifest_files = crate::utils::manifest::get_last_manifest(watch_dir);
    match last_manifest_files {
        Some(last_manifest) => {
            // get current files in the manifest
            let last_files = last_manifest.files_to_map();
            // loop through the files in new manifest and compare them to the last manifest files
            for file in new_manifest.files() {
                if file.name() == MANIFESTFILENAME {
                    // no need to compare manifest file, it will be updated automatically
                    continue;
                }
                let get_last_file = last_files.get(&file.path);

                match get_last_file {
                    Some(f) => {
                        if !file.is_equal(f) {
                            changes.push(file);
                        }
                    }
                    None => {
                        // add file as there is no last file
                        changes.push(file);
                    }
                }
            }
        }
        None => {
            // last manifest wasn't found; we assume that the user is starting the server for the first time, so we add all files in the manifest
            changes.clear();
            log_verbose!("No last manifest found, adding all files to changes");
            changes.extend(new_manifest.files())
        }
    }
    // save new manifest
    new_manifest.save(watch_dir);

    changes.into_iter().map(|f| f.path).collect()
}
