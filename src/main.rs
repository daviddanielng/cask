pub mod args;
mod builder;
mod server;
mod utils;

use clap::Parser;
use directories::ProjectDirs;
use std::sync::OnceLock;

static VERBOSE: OnceLock<bool> = OnceLock::new();
// static RUNCONFIG: OnceLock<utils::config::RunMode> = OnceLock::new();
static FILESAVENAME: &str = "output.run";
static CACHEDIR: OnceLock<String> = OnceLock::new();
static VERSION: &str = env!("CARGO_PKG_VERSION");
static MIN_VERSION: &str = "0.1.0";
fn main() {
    if let Some(proj_dirs) = ProjectDirs::from("com", "daviddanielng", "cask@daviddanielng.xyz") {
        let cache_dir = proj_dirs.cache_dir().to_str().unwrap().to_string();

        CACHEDIR.set(cache_dir).unwrap_or_else(|_| {
            panic!("Failed to set CACHEDIR. This should never happen since it's only set once.");
        });
    } else {
        panic!(
            "Failed to determine cache directory. This should never happen on supported platforms."
        );
    }
    let args = args::Args::parse();

    VERBOSE.set(args.verbose).unwrap_or_else(|_| {
        panic!("Failed to set VERBOSE flag. This should never happen since it's only set once.");
    });
    match args.start {
        args::StartKind::Build {
            input,
            output,
            gzip,
            force,
        } => {
            builder::start_builder(input, output, gzip, force);
        }
        args::StartKind::Serve { config } => {
            server::start_server(config);
        }
        args::StartKind::DevWatch { input, port } => {
            server::dev_serve::start_dev_serve(input, port);
        }
    }

    // let config = utils::config::parse(std::env::args().collect());
    // match config {
    //
    //     RunMode::Builder(builder_config) => {
    //         builder::start_builder(builder_config);
    //     }
    //     RunMode::Server (server_config) => {
    //         server::start_server(server_config);
    //     }
    // }
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
