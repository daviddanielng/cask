use std::{
    collections::HashMap,
    io::{Write, stdin},
    path::Path,
};

use crate::utils::{
    logger::{log_error, log_info, log_verbose, log_warning, log_warning_inline},
    util::{self, exit_with_error, help},
};

#[derive(Clone)]
pub struct BuilderRunConfig {
    pub input_path: String,
    pub output_path: String,
    pub use_gzip: bool,
    pub overwrite: bool,
}
impl BuilderRunConfig {
    pub(crate) fn parse(arguments: Vec<String>) -> BuilderRunConfig {
        {
            let mut config = BuilderRunConfig {
                input_path: String::new(),
                output_path: String::new(),
                use_gzip: true,
                overwrite: false,
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
                        "--no-gzip" => {
                            config.use_gzip = false;
                        }
                        "--overwrite" => {
                            config.overwrite = true;
                        }
                        _ => {
                            log_error(&format!("Unknown argument: {}", arg), None);
                            util::help();
                        }
                    }
                } else {
                    if *folder_last_value {
                        config.input_path = arg.clone();
                        last.insert("--folder", false);
                        log_verbose(&format!(
                            "folder to compress is set to {}",
                            config.input_path
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
                    help();
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
        let path = &self.input_path;
        log_verbose("validating input path ");
        if path.is_empty() {
            log_error(
                "No input path provided. Use --folder <path> to specify the folder to pack.",
                None,
            );
            crate::utils::util::help();
            return false;
        }
        log_verbose("input path valid");
        log_verbose("checking if input path exists");
        if !util::path_exists(path) {
            log_error(
                &format!("the specified input path does not exist: {}", path),
                None,
            );
            return false;
        }
        log_verbose("Path exists");
        log_verbose("Checking if input path is a directory");
        if !util::is_dir(path) {
            log_error(
                &format!("The specified input path is not a directory: {}", path),
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
            let alternate_output_path = Path::new(&self.input_path).join(crate::FILESAVENAME);
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
            if self.overwrite {
                self.remove_existing_output_file();
                log_warning(
                    format!("Output {} exists and will be overwritten", self.output_path).as_str(),
                );
                return true;
            }
            log_warning_inline(
                format!(
                    "file {} already exists and will be overwritten use --overwrite to bypass this. Do you want to proceed? (y/n):",
                    self.output_path
                )
                .as_str(),
            );

            if self.ask_overwrite_permission() {
                log_info(format!("deleting {}", self.output_path).as_str());
                self.remove_existing_output_file();
            } else {
                exit_with_error("Build cancelled.");
            }
        }

        true
    }

    fn remove_existing_output_file(&self) {
        log_info("deleting existing output");
        if util::delete_file(&self.output_path) {
            log_info("file deleted successfully, proceeding with build...");
        } else {
            exit_with_error(format!("Failed to delete existing file {}. Please check your permissions and try again.",  self.output_path).as_str());
        }
    }
    fn ask_overwrite_permission(&self) -> bool {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        match input.as_str() {
            "y\n" | "Y\n" => true,
            "n\n" | "N\n" => false,
            _ => {
                print!("Invalid input, please enter y or n: ");
                let _ = std::io::stdout().flush();
                self.ask_overwrite_permission()
            }
        }
    }
}
