use std::io::stdin;

use crate::utils::{
    logger::{log_info, log_warning},
    util::{self, delete_file, exit_with_error},
};

pub fn start_builder(config: crate::utils::builder_config::BuilderRunConfig) {
    log_info("starting build");
    let output = config.output_path.clone();
    if !validate_output_path(&output) {
        return;
    }
    log_info(format!("proceeding with build, output file will be {}", output).as_str());
    log_info(format!("hashing folder {}...", config.input_path).as_str());
    let vv = crate::utils::hash_folder::hash(&config.input_path);
    util::save_to_file(
        serde_json::to_string(&vv)
            .unwrap_or_else(|e| {
                exit_with_error(format!("Failed to serialize folder hash data: {}", e).as_str())
            })
            .as_bytes(),
        &output,
    );
    // log_info(format!("hashing completed: {:?}", vv).as_str());
}
fn validate_output_path(output: &str) -> bool {
    if util::path_exists(&output) {
        log_warning(
            format!(
                "file {} already exists and will be overwritten. Do you want to proceed? (y/n)",
                output
            )
            .as_str(),
        );
        if ask_overwrite_permission() {
            log_info(format!("deleting {}", output).as_str());
            if delete_file(&output) {
                log_info("file deleted successfully, proceeding with build...");
                return true;
            } else {
                exit_with_error(format!("Failed to delete existing file {}. Please check your permissions and try again.", output).as_str());
            }
        } else {
            log_info("Build cancelled by user.");
            return false;
        }
    }
    true
}
fn ask_overwrite_permission() -> bool {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    match input.as_str() {
        "y\n" | "Y\n" => true,
        "n\n" | "N\n" => false,
        _ => {
            println!("Invalid input, please enter y or n.");
            ask_overwrite_permission()
        }
    }
}
// pub fn start_buildere(arguments: Vec<String>) {
//     let mut folder_path = String::new();
//     let mut last = HashMap::from([("--folder", false)]);
//     println!("{:?}", arguments);
//     for arg in &arguments {
//         log_verbose(&format!("processing arg {}", arg.to_string()));
//         let folder_last_value: &bool = last.get("--folder").unwrap(); //unwrap is safe here because we know the key exists
//         if arg.starts_with("--") {
//             if *folder_last_value {
//                 log_error("The --folder option can only be used once.", None);
//                 return;
//             }
//             match arg.as_str() {
//                 "--folder" => {
//                     last.insert("--folder", true);
//                 }
//                 _ => {
//                     log_error(&format!("Unknown argument: {}", arg), None);
//                     util::help();
//                 }
//             }
//         } else {
//             if *folder_last_value {
//                 folder_path = arg.clone();
//                 if !validate_folder_path(&folder_path.to_string()) {
//                     return;
//                 }
//                 last.insert("--folder", false);
//                 log_verbose(&format!("folder to compress is set to {}", folder_path));
//                 continue;
//             }
//             log_error(&format!("Unexpected value: {}", arg), None);
//             util::help();
//             return;
//         }
//     }

//     println!("Starting builder with folder path: {}", folder_path);
// }
// //     log_info("Starting loader...");
//     let current_exe = std::env::current_exe();
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
