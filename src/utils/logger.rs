pub fn log_error(message: &str, e: Option<&dyn std::error::Error>) {
    println!("[ERROR] {}", message);
    match e {
        Some(error) => println!("Error details: {}", error),
        None => println!("No additional error details provided."),
    }
}
pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}
