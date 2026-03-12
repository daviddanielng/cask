use std::path::Path;
static MAGIC_NUMBER: &[u8; 8] = b"SFS1XV2Z";
static VERSION: &[u8; 8] = b"1.0.0\0\0";
use crate::utils::{
    logger::log_verbose,
    util::{self, exit_with_error},
};
#[derive(Clone)]
pub struct AppendFiles {
    pub zip: String,
    pub zip_info: String,
}
pub fn add_files(temp_dir: &str, files: AppendFiles) -> String {
    let current_exe = std::env::current_exe().unwrap_or_else(|e| {
        exit_with_error(format!("An error occurred while trying to get execuable: {}", e).as_str());
    });
    let exe_str_path = current_exe.to_str().unwrap_or_else(|| {
        exit_with_error("An error occurred while trying to convert executable path to string.");
    });
    let generated_path = Path::new(temp_dir).join("executable");
    let path_str = generated_path.to_str().unwrap_or_else(|| {
        exit_with_error("An error occurred while trying to convert generated path to string.");
    });
    log_verbose("copying executable to temp dir");
    util::copy_file(exe_str_path, &path_str);
    log_verbose("executable copied to temp dir");
    let magic_number = b"SFS";
    for file in files {
        log_verbose(format!("Adding file {} to executable", file).as_str());
        let file_bytes = std::fs::read(file).unwrap();

        util::append_file_to_executable(&path_str, file);
    }
    path_str.to_string()
}
fn add_file_to_executable(exe_path: &str, file_path: &str, magic_number: &[u8; 8]) {
    let file_bytes = std::fs::read(file_path).unwrap_or_else(|e| {
        exit_with_error(
            format!(
                "failed to append file {} to execuatable: error {}",
                file_path, e
            )
            .as_str(),
        );
    });
    let file_size = file_bytes.len() as u64;
}
pub fn read_files() {}
