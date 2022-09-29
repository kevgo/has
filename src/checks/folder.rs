use std::fs;

/// indicates whether a file with the given name exists
pub fn folder(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}
