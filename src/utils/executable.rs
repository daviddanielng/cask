use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
static MAGIC_NUMBER: &[u8; 8] = b"CASKXV2Z";

use crate::utils::{
    macros::{exit_and_error, log_info, log_verbose},
    util,
};

fn get_exe() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|e| {
        exit_and_error!("An error occurred while trying to get executable: {}", e);
    })
}
pub fn build(temp_dir: &str, zip: &str) -> String {
    let current_exe = get_exe();

    let exe_str_path = current_exe.to_str().unwrap_or_else(|| {
        exit_and_error!("An error occurred while trying to convert executable path to string.");
    });

    let new_exe_path = Path::new(temp_dir).join("executable");
    let path_str = new_exe_path.to_str().unwrap_or_else(|| {
        exit_and_error!("An error occurred while trying to convert generated path to string.");
    });

    log_verbose!("copying executable to temp dir");
    util::copy_file(exe_str_path, path_str);
    log_verbose!("executable copied to temp dir");

    log_verbose!("Adding website files to executable");
    let output = OpenOptions::new()
        .append(true)
        .open(path_str)
        .unwrap_or_else(|e| {
            exit_and_error!("failed to open output executable: {}", e);
        });
    // use a buffered writer to reduce the number of write calls which can improve performance when appending large files to the executable.
    let mut writer = BufWriter::with_capacity(1024 * 1024, output);
    append_file_to_executable(&mut writer, zip);
    writer.flush().unwrap_or_else(|e| {
        exit_and_error!("failed to flush output executable: {}", e);
    });
    log_verbose!("Website files added to executable");

    path_str.to_string()
}

fn append_file_to_executable<W: Write>(exe_path: &mut W, file_path: &str) {
    let input = File::open(file_path).unwrap_or_else(|e| {
        exit_and_error!(
            "failed to open file {} for appending to executable: error {}",
            file_path,
            e
        );
    });

    let file_size = input
        .metadata()
        .unwrap_or_else(|e| {
            exit_and_error!(
                "failed to read metadata for file {}: error {}",
                file_path,
                e
            );
        })
        .len();

    let mut reader = BufReader::with_capacity(1024 * 1024, input);
    // copy file bytes to the end of the executable
    std::io::copy(&mut reader, exe_path).unwrap_or_else(|e| {
        exit_and_error!(
            "failed to stream file {} into executable: error {}",
            file_path,
            e
        );
    });

    // Write file size as little-endian u64 followed by the magic number to the end of the executable.
    exe_path
        .write_all(&file_size.to_le_bytes())
        .unwrap_or_else(|e| {
            exit_and_error!(
                "failed to write file size of file {} to executable: error {}",
                file_path,
                e
            );
        });

    // Write magic number to the end of the executable to indicate that it has embedded files.
    exe_path.write_all(MAGIC_NUMBER).unwrap_or_else(|e| {
        exit_and_error!("failed to write magic number to executable: error {}", e);
    });
}
pub fn read_files(config: &crate::args::server::ServerRunConfig) -> (File, String) {
    log_info!("Reading embedded files from executable...");
    let mut exe = std::fs::File::open(get_exe()).unwrap_or_else(|e| {
        exit_and_error!("failed to open executable, {} ", e);
    });
    exe.seek(SeekFrom::End(-16)).unwrap();
    // magic number and file size.
    let mut tail = [0u8; 16];
    log_verbose!("Reading executable tail to find embedded files...");
    exe.read_exact(&mut tail).unwrap_or_else(|e| {
        exit_and_error!("failed to read executable tail, {}", e);
    });
    log_verbose!("Verifying magic number.");
    // Check magic number which should be the last 8 bytes of the tail
    if &tail[8..] != MAGIC_NUMBER {
        exit_and_error!("magic number mismatch, no embedded files found in executable");
    }
    log_verbose!("Magic number matched, embedded files found in executable.");
    // Get file size from the first 8 bytes of the tail which is stored as little-endian u64.
    let file_size = u64::from_le_bytes(tail[0..8].try_into().unwrap());
    preventive_memory_check(file_size, config);
    log_info!(
        "Embedded zip size: {}, extracting zip file",
        util::bytes_to_readable_size(file_size)
    );
    let (mut zip_file, zip_file_path) = create_embedded_zip();
    // seek to the position of the embedded file which is located at the end of the executable before the tail (magic number and file size).
    exe.seek(SeekFrom::End(-16 - file_size as i64))
        .unwrap_or_else(|e| {
            exit_and_error!(
                "failed to seek to embedded file position in executable: {}",
                e
            );
        });
    // use a buffered reader to reduce the number of read calls which can improve performance when extracting large files from the executable.
    let reader = BufReader::with_capacity(1024 * 1024, exe);
    std::io::copy(&mut reader.take(file_size), &mut zip_file).unwrap_or_else(|e| {
        exit_and_error!(
            "failed to extract embedded file from executable to {}: {}",
            zip_file_path,
            e
        );
    });
    log_info!("Embedded files extracted to {}", zip_file_path);
    (zip_file, zip_file_path)
}

fn preventive_memory_check(
    total_file_size: u64,
    config: &crate::args::server::ServerRunConfig,
) {
    match config.cache.mode {
        crate::server::config::cache::ServerCacheMode::Fill => {
            log_verbose!("Cache mode is set to Fill, saving all embedded files to memory.");
            if total_file_size > config.cache.max_memory {
                exit_and_error!(
                    "Cannot not use cache fill mode, the total size of the embedded files ({}) exceeds the configured max-memory ({}). Extraction cannot proceed.",
                    util::bytes_to_readable_size(total_file_size),
                    util::bytes_to_readable_size(config.cache.max_memory)
                );
            }
        }
        crate::server::config::cache::ServerCacheMode::Hit => {
            log_verbose!(
                "Cache mode is set to Hit, files will be saved to memory based on hit frequency."
            );
        }
    }
}

fn create_embedded_zip() -> (File, String) {
    let temp_dir = util::generate_temp_dir();
    let temp_zip_path = Path::new(&temp_dir).join("embedded.zip");
    let zip_str_path = temp_zip_path.to_str().unwrap_or_else(|| {
        exit_and_error!(
            "Failed to convert temporary zip file path to string: {}",
            temp_zip_path.display()
        );
    });
    if util::path_exists(zip_str_path) {
        log_verbose!(
            "Output zip file already exists at {}, deleting existing file.",
            temp_zip_path.display()
        );
        util::delete_file(zip_str_path);
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_zip_path)
        .unwrap_or_else(|e| {
            exit_and_error!(
                "failed to create output zip file at {}: {}",
                temp_zip_path.display(),
                e
            );
        });

    (file, zip_str_path.to_string())
}
