use std::{
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};
static MAGIC_NUMBER: &[u8; 8] = b"SFS1XV2Z";
use crate::utils::{
    logger::log_verbose,
    util::{self, exit_and_error, exit_with_error},
};

fn get_exe() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|e| {
        exit_with_error(format!("An error occurred while trying to get execuable: {}", e).as_str());
    })
}

pub fn build(temp_dir: &str, zip: &str) -> String {
    let current_exe = get_exe();

    let exe_str_path = current_exe.to_str().unwrap_or_else(|| {
        exit_with_error("An error occurred while trying to convert executable path to string.");
    });

    let generated_path = Path::new(temp_dir).join("executable");
    let path_str = generated_path.to_str().unwrap_or_else(|| {
        exit_with_error("An error occurred while trying to convert generated path to string.");
    });

    log_verbose("copying executable to temp dir");
    util::copy_file(exe_str_path, path_str);
    log_verbose("executable copied to temp dir");

    log_verbose("Adding website files to executable");
    let output = OpenOptions::new()
        .append(true)
        .open(path_str)
        .unwrap_or_else(|e| {
            exit_with_error(format!("failed to open output executable: {}", e).as_str())
        });

    let mut writer = BufWriter::with_capacity(1024 * 1024, output); // 1MB write buffer
    append_file_to_executable(&mut writer, zip);
    writer.flush().unwrap_or_else(|e| {
        exit_with_error(format!("failed to flush output executable: {}", e).as_str())
    });
    log_verbose("Website files added to executable");

    path_str.to_string()
}

fn append_file_to_executable<W: Write>(exe_path: &mut W, file_path: &str) {
    let input = File::open(file_path).unwrap_or_else(|e| {
        exit_with_error(
            format!(
                "failed to open file {} for appending to executable: error {}",
                file_path, e
            )
            .as_str(),
        );
    });

    let file_size = input
        .metadata()
        .unwrap_or_else(|e| {
            exit_with_error(
                format!(
                    "failed to read metadata for file {}: error {}",
                    file_path, e
                )
                .as_str(),
            );
        })
        .len();

    let mut reader = BufReader::with_capacity(1024 * 1024, input); // 1MB read buffer

    std::io::copy(&mut reader, exe_path).unwrap_or_else(|e| {
        exit_with_error(
            format!(
                "failed to stream file {} into executable: error {}",
                file_path, e
            )
            .as_str(),
        );
    });

    exe_path
        .write_all(&file_size.to_le_bytes())
        .unwrap_or_else(|e| {
            exit_with_error(
                format!(
                    "failed to write file size of file {} to executable: error {}",
                    file_path, e
                )
                .as_str(),
            );
        });
    exe_path.write_all(MAGIC_NUMBER).unwrap_or_else(|e| {
        exit_with_error(
            format!("failed to write magic number to executable: error {}", e).as_str(),
        );
    });

    // std::io::copy(&mut std::io::Cursor::new(MAGIC_NUMBER), exe_path).unwrap_or_else(|e| {
    //     exit_with_error(
    //         format!(
    //             "failed to write magic number to executable after file {}: error {}",
    //             file_path, e
    //         )
    //         .as_str(),
    //     );
    // });
}
pub fn read_files() {
    let mut exe = std::fs::File::open(get_exe()).unwrap_or_else(|e| {
        exit_and_error("failed to open executable, {} ", e);
    });
    exe.seek(SeekFrom::End(-16)).unwrap();
    // magic number and file size.
    let mut tail = [0u8; 16];
    exe.read_exact(&mut tail).unwrap_or_else(|e|{
        exit_and_error(format!("failed to read executable tail, {}",e).as_str())
    })
}
