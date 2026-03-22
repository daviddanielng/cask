use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
static MAGIC_NUMBER: &[u8; 8] = b"CASKXV2Z";
use crate::utils::{
    logger::{self, log_info, log_verbose},
    macros::{self, exit_and_error},
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

    log_verbose("copying executable to temp dir");
    util::copy_file(exe_str_path, path_str);
    log_verbose("executable copied to temp dir");

    log_verbose("Adding website files to executable");
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
    log_verbose("Website files added to executable");

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
pub fn read_files(to: &str) {
    logger::log_info("Reading embedded files from executable...");
    let mut exe = std::fs::File::open(get_exe()).unwrap_or_else(|e| {
        exit_and_error!("failed to open executable, {} ", e);
    });
    exe.seek(SeekFrom::End(-16)).unwrap();
    // magic number and file size.
    let mut tail = [0u8; 16];
    log_verbose("Reading executable tail to find embedded files...");
    exe.read_exact(&mut tail).unwrap_or_else(|e| {
        exit_and_error!("failed to read executable tail, {}", e);
    });
    log_verbose("Verifying magic number.");
    // Check magic number which should be the last 8 bytes of the tail
    if &tail[8..] != MAGIC_NUMBER {
        exit_and_error!("magic number mismatch, no embedded files found in executable");
    }
    log_verbose("Magic number matched, embedded files found in executable.");
    // Get file size from the first 8 bytes of the tail which is stored as little-endian u64.
    let file_size = u64::from_le_bytes(tail[0..8].try_into().unwrap());
    macros::log_info!(
        "File size of embedded files: {}",
        util::bytes_to_readable_size(file_size)
    );
    log_info("Extracting file");
}
