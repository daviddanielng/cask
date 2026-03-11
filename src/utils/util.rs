use std::io::{Read, Write};
use std::{fs::File, io::copy, path};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use flate2::{Compression, write::GzEncoder};

use crate::utils::logger::log_verbose;

pub fn help() {
    println!();
    println!(
        "Usage: static-files-server [OPTIONS]
--pack     Pack your files into a single executable for easier distribution and deployment.
    --folder <path>             Specify the folder to pack.
    --output <path> Optional    Specify the output file name for the packed executable.
    --no-gzip                    Disable gzip compression for the packed files.
    --overwrite                  Allow overwriting the output file if it already exists.

--serve   Extract and serve files from a packed executable. 

--verbose   Enable verbose logging for more detailed output.
--help     Show this help message and exit.
"
    );
}

pub fn is_dir(path: &str) -> bool {
    std::path::Path::new(path).is_dir()
}
pub fn file_size(path: &str) -> u64 {
    std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)
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
pub fn delete_dir(path: &str) -> bool {
    if !is_dir(path) {
        exit_with_error(format!("{} is not a directory.", path).as_str());
    }
    if !path_exists(path) {
        exit_with_error(format!("Directory {} does not exist.", path).as_str());
    }
    std::fs::remove_dir_all(path).is_ok()
}

pub fn create_dirs(path: &str) -> bool {
    if path_exists(path) {
        exit_with_error(format!("Directory {} already exists.", path).as_str());
    }
    std::fs::create_dir_all(path).is_ok()
}
pub fn copy_file(src: &str, dst: &str) -> bool {
    if !is_file(src) {
        exit_with_error(format!("{} is not a file.", src).as_str());
    }
    if !path_exists(src) {
        exit_with_error(format!("Source file {} does not exist.", src).as_str());
    }
    std::fs::copy(src, dst).is_ok()
}
pub fn generate_temp_dir() -> String {
    let temp_dir = crate::CACHEDIR.get().unwrap_or_else(|| {
        panic!(
            "CACHEDIR is not set. This should never happen since it's set at the start of main."
        );
    });
    let random_string = generate_random_string(12);
    let temp_path = path::Path::new(temp_dir).join(random_string);
    if !create_dirs(temp_path.to_str().unwrap_or("")) {
        exit_with_error(
            format!(
                "Failed to create temporary directory at {}",
                temp_path.display()
            )
            .as_str(),
        );
    }

    temp_path
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "Failed to convert temporary directory path to string: {}",
                temp_path.display()
            );
        })
        .to_string()
}

pub fn gzip_file(from: &str, to: &str) {
    if !is_file(from) || !path_exists(from) {
        exit_with_error(
            format!("unable to zip {} is not a file or it do not exists.", from).as_str(),
        );
    }
    if !is_dir(
        path::Path::new(to)
            .parent()
            .unwrap_or_else(|| {
                panic!(
                    "Failed to determine parent directory for output path: {}",
                    to
                );
            })
            .to_str()
            .unwrap_or(""),
    ) {
        exit_with_error(format!("Output directory for gzip does not exist: {}", to).as_str());
    }
    log_verbose(format!("gzipping file {} to {}", from, to).as_str());
    let mut input = File::open(from).unwrap_or_else(|_| {
        exit_with_error(format!("Failed to open file for gzip: {}", from).as_str());
    });
    let output = File::create(to).unwrap_or_else(|e| {
        exit_with_error(format!("Failed to create gzip output file {}: {}", to, e).as_str());
    });
    let mut encoder = GzEncoder::new(output, Compression::fast());
    copy(&mut input, &mut encoder).unwrap_or_else(|e| {
        exit_with_error(format!("Failed to gzip file {}: {}", from, e).as_str());
    });
    encoder.finish().unwrap_or_else(|e| {
        exit_with_error(
            format!("Failed to finish gzip encoding for file {}: {}", from, e).as_str(),
        );
    });
}
pub fn zip_dir(input_dir: &str, output_path: &str) {
    let file = File::create(output_path).unwrap();
    let mut zip = ZipWriter::new(file);

    let options: SimpleFileOptions =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        // get relative path e.g. "css/style.css" not "/home/user/project/css/style.css"
        let rel_path = entry
            .path()
            .strip_prefix(input_dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/"); // normalize for Windows

        zip.start_file(&rel_path, options).unwrap();

        let mut f = File::open(entry.path()).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        zip.write_all(&buffer).unwrap();
    }

    zip.finish().unwrap();
}
