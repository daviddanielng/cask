use crate::utils::{executable, logger::log_info, macros::exit_and_error, util};
use ctrlc;
use std::path::{Path, PathBuf};
use std::process::Output;
pub static MANIFESTFILENAME: &str = "??cask_manifest-o-??.json";

pub fn start_builder(input: PathBuf, output: PathBuf, gzip: bool, force: bool) {
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl-C handler");

    log_info("starting build");
    let temp_dir = util::generate_temp_dir();
    log_info("making files hash");
    let manifest = crate::utils::hash_folder::hash(
        input.to_str().unwrap(),
        input.to_str().unwrap(),
        gzip,
        temp_dir.as_str(),
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
    // save the manifest to a file in the temp directory so it can be accessed by the executable when it runs.
    let folder_info_safe_str = Path::new(&temp_dir)
        .join(MANIFESTFILENAME)
        .to_string_lossy()
        .to_string();

    util::save_to_file(
        serde_json::to_string(&manifest)
            .unwrap_or_else(|e| {
                exit_and_error!("Failed to serialize folder hash data: {}", e);
            })
            .as_bytes(),
        &folder_info_safe_str,
    );
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
    clean_temp_dir_files(&temp_dir);
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

//     match current_exe {
//         Ok(excutable) => {
//             let file_exists = Path::new(&excutable).exists();
//             if file_exists {
//                 log_info("Starting builder...");
//                 std::fs::copy(
//                     excutable,
//                     "/home/daniel/Documents/Projects/cask/temp/cask",
//                 )
//                 .unwrap();
//                 let file_bytes = std::fs::read(
//                     "/home/daniel/Documents/Projects/cask/temp/test.txt",
//                 )
//                 .unwrap();
//                 let file_size = file_bytes.len() as u64;
//                 const MAGIC: &[u8; 8] = b"SFS12345";
//                 let mut output = OpenOptions::new().append(true).open("/home/daniel/Documents/Projects/cask/temp/cask").unwrap();
//                 output.write_all(&file_bytes).unwrap(); // the file contents
//                 output.write_all(&file_size.to_le_bytes()).unwrap(); // size as 8 bytes
//                 output.write_all(MAGIC).unwrap();
//                 logger::log_info(
//                     "Build completed successfully. You can find the packed executable at: /home/daniel/Documents/Projects/cask/temp/cask; you can run it to start the server.",
//                 );
//             } else {
//                 logger::log_error("Current executable file does not exist.", None);
//             }
//         }
//         Err(e) => logger::log_error("Error getting current executable path", Some(&e)),
//     }
// }
