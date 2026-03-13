use crate::utils::{builder_config::BuilderRunConfig, logger, server_config::ServerRunConfig};

#[derive(Clone)]
pub enum RunMode {
    Server(ServerRunConfig),
    Builder(BuilderRunConfig),
}

pub fn parse(arguments: Vec<String>) -> RunMode {
    if arguments.len() == 1 {
        crate::utils::util::help();
        std::process::exit(0);
    } else {
        let mut arguments = arguments;
        if arguments.contains(&String::from("--verbose")) {
            crate::VERBOSE.set(true).unwrap_or_else(|_| {
                panic!(
                    "Failed to set VERBOSE flag. This should never happen since it's only set once."
                );
            });
            arguments.retain(|arg| arg != "--verbose");
        }
        let c = get_run_mode(arguments);
        crate::RUNCONFIG.set(c.clone()).unwrap_or_else(|_| {
            panic!("Failed to set CONFIG. This should never happen since it's only set once.");
        });
        c
    }
}
fn get_run_mode(arguments: Vec<String>) -> RunMode {
    let first_arg = arguments[1].clone();
    match first_arg.as_str() {
        "--pack" => {
            let config =
                crate::utils::builder_config::BuilderRunConfig::parse(arguments[2..].to_vec());

            RunMode::Builder(config)
        }
        "--serve" => {
            let config =
                crate::utils::server_config::ServerRunConfig::parse(arguments[2..].to_vec());
            RunMode::Server(config)
        }
        "--help" => {
            crate::utils::util::help();
            std::process::exit(0);
        }
        _ => {
            logger::error!("Unknown command: {}", first_arg);
            crate::utils::util::help();
            std::process::exit(1);
        }
    }
}
// pub fn parse(arguments: Vec<String>) -> Self {
//     let mut verbose = false;
//     let mut command = None;
//     let mut command_args = Vec::new();

//     for arg in arguments {
//         if arg == "--verbose" {
//             verbose = true;
//         } else if command.is_none() {
//             command = Some(arg);
//         } else {
//             command_args.push(arg);
//         }
//     }

//     ConfigS {
//         verbose,
//         command,
//         command_args,
//     }
// }
