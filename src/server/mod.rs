use std::io::{Read, Seek, SeekFrom};

pub fn start_server() {
    let mut exe = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();
    exe.seek(SeekFrom::End(-16)).unwrap();
    let mut tail = [0u8; 16];
    exe.read_exact(&mut tail).unwrap();
    assert_eq!(&tail[8..], b"SFS12345");
    let file_size = u64::from_le_bytes(tail[0..8].try_into().unwrap());
    // Seek to where file bytes start
    exe.seek(SeekFrom::End(-(16 + file_size as i64))).unwrap();

    let mut file_bytes = vec![0u8; file_size as usize];
    exe.read_exact(&mut file_bytes).unwrap();

    println!("{}", String::from_utf8(file_bytes).unwrap());
}
