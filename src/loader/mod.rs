use std::path::Path;

use crate::utils::logger;

pub fn start_loader() {
    logger::log_info("Starting loader...");
    let current_exe = std::env::current_exe();
    match current_exe {
        Ok(exe) => {
            let file_exists = Path::new(&exe).exists();
            if file_exists {
                logger::log_info("Current executable file exists.");
                std::fs::copy(
                    exe,
                    "/home/daniel/Documents/Projects/static-files-server/temp/static-files-server",
                )
                .unwrap();
            } else {
                logger::log_error("Current executable file does not exist.", None);
            }
        }
        Err(e) => logger::log_error("Error getting current executable path", Some(&e)),
    }
}
