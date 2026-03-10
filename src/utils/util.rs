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
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

pub fn zip_dir() -> Option<String> {
    None
}
