use std::fs;

/// indicates whether a file with the given name exists
pub fn file(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}
