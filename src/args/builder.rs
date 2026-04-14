//! ```rust
//! /*!
//! Validates input and output paths to ensure they meet specific requirements.
//!
//! # Functions
//!
//! ## `validate_input`
//!
//! Validates the given input path as a directory. Ensures that the path exists and is a directory.
//!
//! ### Parameters
//! - `s: &str` - A string representing the input path to validate.
//!
//! ### Returns
//! - `Ok(PathBuf)` - If the input path exists and is a directory, returns the `PathBuf` representation of the path.
//! - `Err(String)` - If the input path does not exist or is not a directory, returns an error message with details.
//!
//! ### Errors
//! - Returns an error if the input path does not exist.
//! - Returns an error if the input path is not a directory.
//!
//! ---
//!
//! ## `validate_output`
//!
//! Validates the given output path as a file. Ensures that the output path does not reference a directory, and that its parent directory exists.
//!
//! ### Parameters
//! - `s: &str` - A string representing the output path to validate.
//!
//! ### Returns
//! - `Ok(PathBuf)` - If the output path is valid and its parent directory exists, returns the `PathBuf` representation of the path.
//! - `Err(String)` - If the output path is invalid, returns an error message with details.
//!
//! ### Errors
//! - Returns an error if the output path refers to a directory.
//! - Returns an error if the parent directory of the output path does not exist.
//!
//! ---
//!
//! # Example Usage
//!
//! ```rust
//! use std::path::PathBuf;
//!
//! fn main() {
//!     // Example: Validating an input directory
//!     match validate_input("/path/to/input") {
//!         Ok(path) => println!("Valid input directory: {:?}", path),
//!         Err(err) => eprintln!("Error: {}", err),
//!     }
//!
//!     // Example: Validating an output file
//!     match validate_output("/path/to/output/file.txt") {
//!         Ok(path) => println!("Valid output file: {:?}", path),
//!         Err(err) => eprintln!("Error: {}", err),
//!     }
//! }
//! ```
//! */
//! ```
use std::path::PathBuf;

pub fn validate_input(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        if !&path.is_dir() {
            return Err(format!("Input must be a directory: {}", s));
        }
        Ok(path)
    } else {
        Err(format!("Input does not exist: {}", s))
    }
}

pub fn validate_output(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        if path.is_dir() {
            return Err(format!("Output must be a file: {}", s));
        }
    }
    let parent = path.parent().unwrap();
    if !parent.exists() {
        return Err(format!(
            "Output parent directory does not exist: {:?}",
            parent
        ));
    }
    Ok(path)
}
