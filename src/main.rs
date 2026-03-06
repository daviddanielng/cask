mod loader;
mod server;
mod utils;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        server::start_server();
        println!("No arguments passed, starting server...");
    } else {
        if !args.contains(&String::from("--build")) {
            loader::start_loader();
        } else {
            utils::logger::log_info(
                "Build argument passed, skipping loader and server to allow build process to complete without interference.",
            );
        }
    }
}
