use crate::utils::{
    executable,
    logger::log_info,
    util::{self, exit_with_error},
};
use ctrlc;

pub fn start_builder(config: crate::utils::builder_config::BuilderRunConfig) {
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl-C handler");

    log_info("starting build");
    let output = config.output_path.clone();
    log_info(format!("proceeding with build, output file will be {}", output).as_str());
    let temp_dir = util::generate_temp_dir();
    let vv = crate::utils::hash_folder::hash(
        &config.input_path,
        &config.input_path,
        config.use_gzip,
        temp_dir.as_str(),
    );
    if rx.try_recv().is_ok() {
        log_info("Build cancelled by user (Ctrl+C)");
        clean_temp_dir_files(&temp_dir);
        return;
    }
    util::save_to_file(
        serde_json::to_string(&vv)
            .unwrap_or_else(|e| {
                exit_with_error(format!("Failed to serialize folder hash data: {}", e).as_str())
            })
            .as_bytes(),
        format!("{}.json", output).as_str(),
    );
    util::zip_dir(&vv.path, format!("{}.zip", &vv.path).as_str());
    util::copy_file(
        format!("{}.zip", &vv.path).as_str(),
        format!("{}.zip", &config.output_path).as_str(),
    );
    let exe_path = build_exe(temp_dir.as_str());
    util::copy_file(
        exe_path.as_str(),
        format!("{}.run", &config.output_path).as_str(),
    );
    clean_temp_dir_files(&temp_dir);
}

fn build_exe(temp_dir: &str) -> String {
    log_info("building executable");
    executable::add_files(temp_dir)
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
//                     "/home/daniel/Documents/Projects/static-files-server/temp/static-files-server",
//                 )
//                 .unwrap();
//                 let file_bytes = std::fs::read(
//                     "/home/daniel/Documents/Projects/static-files-server/temp/test.txt",
//                 )
//                 .unwrap();
//                 let file_size = file_bytes.len() as u64;
//                 const MAGIC: &[u8; 8] = b"SFS12345";
//                 let mut output = OpenOptions::new().append(true).open("/home/daniel/Documents/Projects/static-files-server/temp/static-files-server").unwrap();
//                 output.write_all(&file_bytes).unwrap(); // the file contents
//                 output.write_all(&file_size.to_le_bytes()).unwrap(); // size as 8 bytes
//                 output.write_all(MAGIC).unwrap();
//                 logger::log_info(
//                     "Build completed successfully. You can find the packed executable at: /home/daniel/Documents/Projects/static-files-server/temp/static-files-server; you can run it to start the server.",
//                 );
//             } else {
//                 logger::log_error("Current executable file does not exist.", None);
//             }
//         }
//         Err(e) => logger::log_error("Error getting current executable path", Some(&e)),
//     }
// }
