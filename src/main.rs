mod builder;
mod server;
mod utils;
use std::sync::OnceLock;

use directories::ProjectDirs;

use crate::utils::config::RunMode;
static VERBOSE: OnceLock<bool> = OnceLock::new();
static RUNCONFIG: OnceLock<utils::config::RunMode> = OnceLock::new();
static FILESAVENAME: &str = "output.run";
static CACHEDIR: OnceLock<String> = OnceLock::new();

// static VERSION: &str = env!("CARGO_PKG_VERSION");
// pub static mut VERBOSE: bool = false;
fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "daviddanielng", "stsf") {}
    let config = utils::config::parse(std::env::args().collect());
    match config {
        RunMode::Builder(builder_config) => {
            builder::start_builder(builder_config);
        }
        RunMode::Server => {
            server::start_server();
        }
    }
}
// utils::logger::log_info(&format!("Starting Static Files Server version {}", VERSION));
// let mut args = std::env::args().collect::<Vec<String>>();
// if args.contains(&String::from("--verbose")) {
//     VERBOSE.set(true).unwrap_or_else(|_| {
//         panic!(
//             "Failed to set VERBOSE flag. This should never happen since it's only set once."
//         );
//     });
//     args.retain(|arg| arg != "--verbose");
// }
// if args.len() == 1 {
//     println!("No arguments passed, starting server...");
// } else {
//     let first_arg = &args[1];
//     match first_arg.as_str() {
//         "--pack" => {}
//         _ => {
//             utils::util::help();
//         }
//     }
// }
