pub(crate) mod builder;
pub mod server;

use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser, Clone)]

pub enum StartKind {
    #[command(name = "build", about = "Build the site")]
    Build {
        #[arg(short = 'i', long = "input", help = "The input dir to build", value_parser = builder::validate_input)]
        input: PathBuf,
        #[arg(short = 'o', long = "output", help = "The file to output to", value_parser = builder::validate_output)]
        output: PathBuf,
        #[arg(
            short = 'g',
            long = "gzip",
            help = "Whether to gzip the output file",
            default_value = "true"
        )]
        gzip: bool,
        #[arg(
            short = 'f',
            long = "force",
            help = "Whether to overwrite the output file if it exists",
            default_value = "false"
        )]
        force: bool,
    },
    #[command(name = "serve", about = "Serve the site")]
    Serve {
        #[arg(short = 'c', long = "config", help = "The config file", value_parser =
        server::ServerRunConfig::parse)]
        config: server::ServerRunConfig,
    },
}
#[derive(Parser, Debug)]
#[command(name = "cask", about = "A CLI tool for wrapping static web files to one execuable", long_about = None,version,about)]
pub struct Args {
    #[command(subcommand)]
    pub start: StartKind,
    #[arg(short = 'v', long = "verbose", help = "Enable verbose logging", action =
    clap::ArgAction::SetTrue,        global = true)
    ]
    pub verbose: bool,
}
