pub fn log_error(message: &str, e: Option<&dyn std::error::Error>) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";

    println!("{RED}[ERROR] {message}{RESET}");
    match e {
        Some(error) => println!("{RED}Error details: {error}{RESET}"),
        None => {}
    }
}
pub fn log_warning(message: &str) {
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";

    println!("{YELLOW}[WARNING] {message}{RESET}");
}
pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn log_verbose(message: &str) {
    if crate::VERBOSE.get().copied().unwrap_or(false) {
        println!("[VERBOSE] {}", message);
    }
}
