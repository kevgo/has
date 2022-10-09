use glob::glob;
use std::fs;

/// indicates whether a file with the given name exists
pub fn file(pattern: &str) -> bool {
    let entries = match glob(pattern) {
        Ok(paths) => paths,
        Err(_) => return false,
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return false,
        };
        let metadata = match fs::metadata(entry) {
            Ok(metadata) => metadata,
            Err(_) => return false,
        };
        if metadata.is_file() {
            return true;
        }
    }
    false
}
