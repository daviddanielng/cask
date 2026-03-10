use std::{collections::HashMap, path::Path};

use crate::utils::{
    logger::{log_error, log_info, log_verbose, log_warning},
    util::{self, exit_with_error},
};

#[derive(Clone)]
pub struct BuilderRunConfig {
    pub folder_path: String,
    pub output_path: String,
}
impl BuilderRunConfig {
    pub(crate) fn parse(arguments: Vec<String>) -> BuilderRunConfig {
        {
            let mut config = BuilderRunConfig {
                folder_path: String::new(),
                output_path: String::new(),
            };
            let mut last = HashMap::from([("--folder", false), ("--output", false)]);
            for arg in arguments {
                let folder_last_value: &bool = last.get("--folder").unwrap(); //unwrap is safe here because we know the key exists
                let output_last_value: &bool = last.get("--output").unwrap(); //unwrap is safe here because we know the key exists
                log_verbose(&format!("processing arg {}", arg.to_string()));
                if arg.starts_with("--") {
                    if *folder_last_value {
                        exit_with_error("path is required after --folder.");
                    }
                    if *output_last_value {
                        exit_with_error("path is required after --output.");
                    }
                    match arg.as_str() {
                        "--folder" => {
                            last.insert("--folder", true);
                        }
                        "--output" => {
                            last.insert("--output", true);
                        }
                        _ => {
                            log_error(&format!("Unknown argument: {}", arg), None);
                            util::help();
                        }
                    }
                } else {
                    if *folder_last_value {
                        config.folder_path = arg.clone();

                        last.insert("--folder", false);
                        log_verbose(&format!(
                            "folder to compress is set to {}",
                            config.folder_path
                        ));
                        continue;
                    }
                    if *output_last_value {
                        config.output_path = arg.clone();

                        log_verbose(&format!(
                            "output file name is set to {}",
                            config.output_path
                        ));
                        last.insert("--output", false);
                        continue;
                    }
                    exit_with_error(&format!("Unexpected value: {}", arg));
                }
            }
            if !config.validate() {
                exit_with_error("Invalid configuration. Please fix the errors and try again.");
            }
            config
        }
    }
    fn validate(&mut self) -> bool {
        self.validate_input_dir_path() && self.validate_output_dir_path()
    }

    fn validate_input_dir_path(&self) -> bool {
        let path = &self.folder_path;
        log_verbose("validating folder path ");
        if path.is_empty() {
            log_error(
                "No folder path provided. Use --folder <path> to specify the folder to pack.",
                None,
            );
            crate::utils::util::help();
            return false;
        }
        log_verbose("dir path valid");
        log_verbose("Checking if folder path exists");
        if !util::path_exists(path) {
            log_error(
                &format!("The specified folder path does not exist: {}", path),
                None,
            );
            return false;
        }
        log_verbose("Path exists");
        log_verbose("Checking if folder path is a directory");
        if !util::is_dir(path) {
            log_error(
                &format!("The specified path is not a directory: {}", path),
                None,
            );
            return false;
        }
        log_verbose("Path is a directory");
        true
    }
    fn validate_output_dir_path(&mut self) -> bool {
        let path = &self.output_path;
        if path.is_empty() {
            let alternate_output_path = Path::new(&self.folder_path).join(crate::FILESAVENAME);
            log_info(&format!(
                "Output path not provided, using default output file name: {}",
                alternate_output_path.display()
            ));
            self.output_path = alternate_output_path.to_string_lossy().to_string();
        }
        if util::path_exists(&self.output_path) {
            if util::is_dir(&self.output_path) {
                log_error(
                    &format!(
                        "The specified output path is a directory, it must be a file: {}",
                        self.output_path
                    ),
                    None,
                );
                return false;
            }
            log_warning("Output path already exists, it will be overwritten.");
        }

        true
    }
}
