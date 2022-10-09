use glob::glob;
use std::fs;

use crate::errors::UserError;

/// indicates whether a file with the given name exists
pub fn file(pattern: String) -> Result<bool, UserError> {
    let entries = glob(&pattern).map_err(|err| UserError::InvalidGlob {
        pattern,
        guidance: err.to_string(),
    })?;
    for entry in entries {
        let entry = entry.map_err(|err| UserError::CannotReadPath {
            path: err.path().into(),
            guidance: err.to_string(),
        })?;
        let metadata = fs::metadata(&entry).map_err(|err| UserError::CannotReadPath {
            path: entry,
            guidance: err.to_string(),
        })?;
        if metadata.is_file() {
            return Ok(true);
        }
    }
    Ok(false)
}
