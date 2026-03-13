use serde::Deserialize;
use std::{collections::HashMap, io::Write};

use crate::utils::{macros, util};

#[derive(Clone, Deserialize)]
pub struct ServerRunConfig {
    pub output: String,
    pub overwrite: bool,
    #[serde(default = "default_port")]
    pub port: u16,
}
fn default_port() -> u16 {
    7997
}
impl ServerRunConfig {
    pub(crate) fn parse(arguments: Vec<String>) -> ServerRunConfig {
        if arguments.len() == 0 {
            util::help();

            macros::exit_and_error!("No arguments provided for server mode.");
        }
        if arguments.len() > 2 {
            util::help();

            macros::exit_and_error!("Too many arguments provided for server mode.");
        }
        let config_file = arguments[0].clone();
        if !ServerRunConfig::validate_config(&config_file) {
            util::help();

            macros::exit_and_error!("Invalid config file provided for server mode.");
        }

        let contents = std::fs::read_to_string(config_file).unwrap_or_else(|e| {
            macros::exit_and_error!("Failed to read config file: {}", e);
        });
        serde_yaml::from_str(&contents).unwrap_or_else(|e| {
            macros::exit_and_error!("Failed to parse config file: {}", e);
        })
        // config
    }
    fn validate_config(config: &str) -> bool {
        if !util::path_exists(config) {
            macros::log_error!("Config file does not exist: {}", config);
            return false;
        }
        if !util::is_file(config) {
            macros::log_error!("Config path is not a file: {}", config);
            return false;
        }
        if !util::is_file_extension(config, "yaml") {
            macros::log_error!("Config file must be a .yaml file: {}", config);
            return false;
        }
        macros::log_verbose!("Config file path is valid");
        true
    }
}

//     fn validate_output(&self) -> bool {
//         let path = &self.output_path;
//         if path.is_empty() {
//             macros::log_error!(
//                 "Output file path is required. Use --config <path> to specify the config file."
//             );
//             util::help();
//         }
//         if !util::path_exists(path) {
//             macros::log_error!("Output file does not exist: {}", path);
//             return false;
//         }
//         macros::log_verbose!("Output file path is valid");
//         if !util::is_dir(path) {
//             macros::log_error!("Output directory does not exist: {}", path);
//             return false;
//         }
//         let overwrite = self.overwrite;
//         let output_has_content = util::dir_has_content(path);
//         if output_has_content && !overwrite {
//             if self.ask_overwrite_permission() {
//                 macros::log_warning!("User chose to overwrite the output directory.");
//             } else {
//                 macros::exit_and_error!(
//                     "Output directory is not empty and overwrite is not enabled. Exiting."
//                 );
//             }
//         } else if output_has_content && overwrite {
//             macros::log_warning!(
//                 "Output directory is not empty, but overwrite is enabled, so it will be overwritten, files not in embedded files will be deleted."
//             );
//         } else {
//             macros::log_verbose!("Output directory is empty.");
//         }

//         true
//     }
//     fn ask_overwrite_permission(&self) -> bool {
//         let mut input = String::new();
//         std::io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");
//         match input.as_str() {
//             "y\n" | "Y\n" => true,
//             "n\n" | "N\n" => false,
//             _ => {
//                 print!("Invalid input, please enter y or n: ");
//                 let _ = std::io::stdout().flush();
//                 self.ask_overwrite_permission()
//             }
//         }
//     }
// }
