pub enum MemoryFormat {
    Bytes,
    KB,
    MB,
    GB,
}
pub fn total_memory() -> u64 {
    total_memory_with_format(None)
}
fn bytes_to_format(bytes:u64,format:Option<MemoryFormat>)->u64{
    match format.unwrap(){
        MemoryFormat::Bytes => bytes,
        MemoryFormat::KB => bytes / 1024,
        MemoryFormat::MB => bytes / (1024 * 1024),
        MemoryFormat::GB => bytes / (1024 * 1024 * 1024),
    }
}
pub fn total_memory_with_format(format: Option<MemoryFormat>) -> u64 {
    let bytes = sysinfo::System::new_all().total_memory();
    bytes_to_format(bytes, format)
}
pub fn free_memory() -> u64 {
    free_memory_with_format(None)
}

pub fn free_memory_with_format(format: Option<MemoryFormat>) -> u64 {
    let bytes = sysinfo::System::new_all().available_memory();
    bytes_to_format(bytes,format)
}
