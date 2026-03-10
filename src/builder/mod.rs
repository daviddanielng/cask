use crate::utils::{logger::log_warning, util};

pub fn start_builder(config: crate::utils::builder_config::BuilderRunConfig) {
    let mut wait_for_input = false;
    if util::path_exists(&config.output_path) {
        wait_for_input = true;
        log_warning(
            format!(
                "file {} already exists and will be overwritten. Do you want to proceed? (y/n)",
                config.output_path
            )
            .as_str(),
        );
        while wait_for_input {
            
        }

    }
}
pub fn ask_overwrite_permission(){
    
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
