use crate::utils::{executable, logger::log_info, macros::exit_and_error, util};
use ctrlc;
use std::path::{Path, PathBuf};
use std::process::Output;
pub static MANIFESTFILENAME: &str = "??cask_manifest-o-??.json";
pub static ROUTESFILENAME: &str = "??cask_routes-o-??.json";

pub fn start_builder(input: PathBuf, output: PathBuf, gzip: bool, force: bool) {
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl-C handler");

    log_info("starting build");
    // make temp dir, this is what we zip.
    let temp_dir = util::generate_temp_dir();
    log_info("making files hash");
    let manifest = crate::utils::manifest::get_manifest(
        input.to_str().unwrap(),
        temp_dir.as_str(), gzip,
    );
    log_info("files hashed successfully");
    if rx.try_recv().is_ok() {
        // if we receive a signal, it means the user wants to cancel the build
        log_info("Build cancelled by user (Ctrl+C)");
        clean_temp_dir_files(&temp_dir);
        return;
    }

    let zip_path = Path::new(format!("{}.zip", temp_dir).as_str())
        .to_string_lossy()
        .to_string();
  
    // save the manifest to a file in the temp directory so it can compressd with emmbbedded files.
   let manifest_save_location= manifest.save(&temp_dir);
    
    println!("Find manifest at {}",manifest_save_location);
    log_info("zipping web files");
    util::zip_dir(&manifest.path, zip_path.as_str());
    log_info("web files zipped successfully");
    log_info("building executable");
    let exe_path = executable::build(temp_dir.as_str(), zip_path.as_str());
    util::copy_file(
        exe_path.as_str(),
        format!("{}.run", output.to_str().unwrap()).as_str(),
    );
    log_info("executable built");
    // clean_temp_dir_files(&temp_dir);
    log_info(format!("Build completed successfully. You can find the packed executable at: {}.run; you can run it to start the server.", output.to_str().unwrap()).as_str());
}

fn clean_temp_dir_files(temp_dir: &str) {
    log_info(format!("Cleaning up temporary directory {} ", temp_dir).as_str());
    if !util::delete_dir(temp_dir) {
        log_info(format!("Failed to delete temporary directory at {}. Please check your permissions and delete it manually.", temp_dir).as_str());
    }
    if util::path_exists(format!("{}.zip", temp_dir).as_str()) {
        log_info(
            format!(
                "Cleaning up temporary zipped file {} ",
                format!("{}.zip", temp_dir)
            )
            .as_str(),
        );

        if !util::delete_file(format!("{}.zip", temp_dir).as_str()) {
            log_info(format!("Failed to delete temporary zipped file at {}. Please check your permissions or delete it manually.", format!("{}.zip", temp_dir)).as_str());
        }
    }
}