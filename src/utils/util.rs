use std::io::{Read, Write};
use std::{fs::File, io, io::copy, path};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::FileOptions};

use crate::utils::macros::{exit_and_error, log_error, log_verbose};
use flate2::{Compression, write::GzEncoder};
use xxhash_rust::xxh3::Xxh3;
use zip::result::ZipError;



pub fn dir_has_content(path: &str) -> bool {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .any(|e| e.path().is_file())
}
pub fn is_file_extension(path: &str, extension: &str) -> bool {
    path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or(false, |ext_str| ext_str.eq_ignore_ascii_case(extension))
}
pub fn bytes_to_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
pub fn is_dir(path: &str) -> bool {
    path::Path::new(path).is_dir()
}
pub fn file_size(path: &str) -> u64 {
    std::fs::metadata(path).map(|meta| meta.len()).unwrap_or(0)
}
pub fn is_file(path: &str) -> bool {
    path::Path::new(path).is_file()
}
pub fn path_exists(path: &str) -> bool {
    path::Path::new(path).exists()
}
#[deprecated(since = "0.1.0", note = "Use macro exit_and_error instead")]
pub fn exit_with_error(message: &str) -> ! {
    eprintln!("\x1b[31mError: {}\x1b[0m", message);
    std::process::exit(1);
}

pub fn save_to_file(content: &[u8], path: &str) -> bool {
    std::fs::write(path, content).is_ok()
}
pub fn hash_file(path: &str) -> io::Result<u64> {
    let mut file = File::open(path)?;
    let mut hasher = Xxh3::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.digest())
}
pub fn generate_random_string(length: usize) -> String {
    use rand::distr::{Alphanumeric, SampleString};
    let mut rng = rand::rng();
    Alphanumeric.sample_string(&mut rng, length)
}
/// Delete a file or directory at the given path.
/// This function will check if the path exists and if it's a file or directory before attempting to delete it. If the path does not exist or is not the expected type, an error will be logged and the program will exit.
/// # Arguments
/// * `path` - The path to the file or directory to delete.
/// # Returns
/// true if the file or directory was successfully deleted, false otherwise.
pub fn delete_file(path: &str) -> bool {
    if !is_file(path) {
        exit_and_error!("{} is not a file.", path);
    }
    if !is_file(path) {
        exit_and_error!("{} is not a file.", path);
    }
    if !path_exists(path) {
        exit_and_error!("File {} does not exist.", path);
    }
    std::fs::remove_file(path).is_ok()
}
pub fn delete_dir(path: &str) -> bool {
    if !is_dir(path) {
        exit_and_error!("{} is not a directory.", path);
    }
    if !path_exists(path) {
        exit_and_error!("Directory {} does not exist.", path);
    }
    std::fs::remove_dir_all(path).is_ok()
}

pub fn create_dirs(path: &str) -> bool {
    if path_exists(path) {
        exit_and_error!("Directory {} already exists.", path);
    }
    std::fs::create_dir_all(path).is_ok()
}
pub fn create_dirs_not_existing(path: &str) -> bool {
    if path_exists(path) {
        log_verbose!("Path {} already exists.", path);
        return true;
    }
    let create = std::fs::create_dir_all(path);
    match create {
        Ok(_) => true,
        Err(e) => {
            log_error!("Error creating directory {}: {}", path, e);
            false
        }
    }
}
pub fn copy_file(src: &str, dst: &str) -> bool {
    std::fs::copy(src, dst).unwrap_or_else(|e| {
        exit_and_error!("Failed to copy {} to {}: {}", src, dst, e);
    });
    true
}
pub fn generate_temp_dir() -> String {
    let temp_dir = crate::CACHEDIR.get().unwrap_or_else(|| {
        panic!(
            "CACHEDIR is not set. This should never happen since it's set at the start of main."
        );
    });
    let random_string = generate_random_string(12);
    let mut temp_path = path::Path::new(temp_dir).join(random_string);
    while path_exists(temp_path.to_str().unwrap_or_else(|| {
        exit_and_error!(
            "Failed to convert temporary directory path to string: {}",
            temp_path.display()
        );
    })) {
        let random_string = generate_random_string(12);
        temp_path = path::Path::new(temp_dir).join(random_string);
    }
    if !create_dirs(temp_path.to_str().unwrap_or("")) {
        exit_and_error!(
            "Failed to create temporary directory at {}",
            temp_path.display()
        );
    }

    temp_path
        .to_str()
        .unwrap_or_else(|| {
            exit_and_error!(
                "Failed to convert temporary directory path to string: {}",
                temp_path.display()
            );
        })
        .to_string()
}

pub fn gzip_file(from: &str, to: &str) {
    if !is_file(from) || !path_exists(from) {
        exit_and_error!("unable to zip {} is not a file or it do not exists.", from);
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
        exit_and_error!("Output directory for gzip does not exist: {}", to);
    }
    log_verbose!("gzipping file {} to {}", from, to);
    let mut input = File::open(from).unwrap_or_else(|_| {
        exit_and_error!("Failed to open file for gzip: {}", from);
    });
    let output = File::create(to).unwrap_or_else(|e| {
        exit_and_error!("Failed to create gzip output file {}: {}", to, e);
    });
    let mut encoder = GzEncoder::new(output, Compression::fast());
    copy(&mut input, &mut encoder).unwrap_or_else(|e| {
        exit_and_error!("Failed to gzip file {}: {}", from, e);
    });
    encoder.finish().unwrap_or_else(|e| {
        exit_and_error!("Failed to finish gzip encoding for file {}: {}", from, e);
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
        // get a relative path e.g. "css/style.css" not "/home/user/project/css/style.css"
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

    zip.finish().unwrap_or_else(|e| {
        exit_and_error!("Failed to finish creating zip file {}: {}", output_path, e);
    });
}

pub fn file_exists_in_zip(zip_path: &str, file_name: &str) -> bool {
    let file = File::open(zip_path).unwrap_or_else(|e| {
        exit_and_error!("Failed to open zip file {}: {}", zip_path, e);
    });
    let zip = ZipArchive::new(file).unwrap_or_else(|e| {
        exit_and_error!("Failed to read zip file {}: {}", zip_path, e);
    });
    zip.file_names().any(|name| name == file_name)
}

pub fn extract_from_zip(zip: &File, file_path: &str) -> Result<Vec<u8>, ZipError> {
    log_verbose!("Extracting {} from zip", file_path);
    let mut archive = ZipArchive::new(zip)?;
    let mut file = archive.by_name(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
pub fn file_mime(path: &str)->mime::Mime {
    // TODO: add support for more content types
    let file_mime;
    let file_extension = path.split('.').last().unwrap_or("");
    match file_extension {
        "html" => {
            file_mime = mime::TEXT_HTML;
        }
        "js" => {
            file_mime = mime::TEXT_JAVASCRIPT;
        }
        "css" => {
            file_mime = mime::TEXT_CSS;
        }
        "png" => {
            file_mime = mime::IMAGE_PNG;
        }
        "svg" => {
            file_mime = mime::IMAGE_SVG;
        }
        "jpeg" | "jpg" => {
            file_mime = mime::IMAGE_JPEG;
        }
        _ => {
            file_mime = mime::TEXT_PLAIN;
        }
    }
    file_mime
}
