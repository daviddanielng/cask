pub fn help() {
    println!();
    println!(
        "Usage: static-files-server [OPTIONS]
--pack     Pack your files into a single executable for easier distribution and deployment.
    --folder <path>             Specify the folder to pack.
    --output <path> Optional     Specify the output file name for the packed executable. .

--serve   Extract and serve files from a packed executable. 


--help     Show this help message and exit.
"
    );
}

pub fn is_dir(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}

pub fn is_file(path: &str) -> bool {
    std::path::Path::new(path).is_file()
}
pub fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn exit_with_error(message: &str) -> ! {
    eprintln!("\x1b[31mError: {}\x1b[0m", message);
    std::process::exit(1);
}

pub fn zip_dir() -> Option<String> {
    None
}
pub fn save_to_file(content: &[u8], path: &str) -> bool {
    std::fs::write(path, content).is_ok()
}

pub fn generate_random_string(length: usize) -> String {
    use rand::distr::{Alphanumeric, SampleString};
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length)
}

pub fn delete_file(path: &str) -> bool {
    if !is_file(path) {
        exit_with_error(format!("{} is not a file.", path).as_str());
    }
    if !path_exists(path) {
        exit_with_error(format!("File {} does not exist.", path).as_str());
    }
    std::fs::remove_file(path).is_ok()
}
