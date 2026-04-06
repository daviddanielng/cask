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
    if !parent.exists(){
        return Err(format!("Output parent directory does not exist: {:?}", parent));
    }
    Ok(path)
}
