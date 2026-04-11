use crate::server::config::cache::ServerCache;
use crate::utils::{macros, util};
use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize, Debug)]
pub struct ServerRunConfig {
    #[serde(deserialize_with = "deserialize_output_dir")]
    pub output: String,
    #[serde(default = "default_port", deserialize_with = "deserialize_port")]
    pub port: u16,
    pub cache: ServerCache,
    #[serde(default = "default_fallback")]
    pub fallback: Option<String>,
}

impl ServerRunConfig {
    pub(crate) fn parse(config_file: &str) -> Result<ServerRunConfig, String> {
        if !ServerRunConfig::validate_config(&config_file) {
            return Err(format!("Invalid config : {}", config_file));
        }

        let contents = std::fs::read_to_string(config_file);
        serde_yaml::from_str::<ServerRunConfig>(&contents.unwrap())
            .map_err(|e| format!("Failed to parse config file: {}", e))

        // match contents {
        //     Err(e) => {
        //         return Err(format!("Failed to read config file: {}", e));
        //     }
        //     Ok(contents) => {
        //         println!("{}",contents);
        //         serde_yaml::from_str(&contents).unwrap_or_else(|e| {
        //             return Err(format!("Failed to parse config file: {}", e));
        //         })
        //     },
        // }
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
fn default_fallback() -> Option<String> {
    None
}

fn default_port() -> u16 {
    7997
}
fn deserialize_output_dir<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let output = String::deserialize(deserializer)?;
    if output.is_empty() {
        return Err(serde::de::Error::custom(
            "Output directory path cannot be empty.",
        ));
    }
    if !util::path_exists(&output) {
        return Err(serde::de::Error::custom(format!(
            "`output` path does not exist: {}",
            output
        )));
    }
    if !util::is_dir(&output) {
        return Err(serde::de::Error::custom(format!(
            "`output` must be a directory: {}",
            output
        )));
    }
    if util::dir_has_content(&output) {
        macros::log_warning!(
            "Output directory {} is not empty. all files in the directory will be deleted.",
            output
        );
    }
    if !output.ends_with("/"){
        return Err(serde::de::Error::custom(
            "`output` must end with '/'.",
        ))
    }

    Ok(output)
}
fn deserialize_port<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let port = u16::deserialize(deserializer)?;
    if port == 0 {
        return Err(serde::de::Error::custom(
            "Port number must be between 1 and 65535.",
        ));
    }
    Ok(port)
}
