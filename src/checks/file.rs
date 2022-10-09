use crate::errors::UserError;
use glob::glob;
use regex::Regex;
use std::fs;

/// indicates whether a file with the given name exists
pub fn file(pattern: String, contains: Option<String>) -> Result<bool, UserError> {
    let entries = glob(&pattern).map_err(|err| UserError::InvalidGlob {
        pattern,
        guidance: err.to_string(),
    })?;
    let regex = match contains {
        Some(pattern) => match Regex::new(&pattern) {
            Ok(regex) => Some(regex),
            Err(err) => {
                return Err(UserError::InvalidRegex {
                    pattern,
                    guidance: err.to_string(),
                })
            }
        },
        None => None,
    };
    for entry in entries {
        let entry = entry.map_err(|err| UserError::CannotReadPath {
            path: err.path().into(),
            guidance: err.to_string(),
        })?;
        let metadata = fs::metadata(&entry).map_err(|err| UserError::CannotReadPath {
            path: entry.clone(),
            guidance: err.to_string(),
        })?;
        if !metadata.is_file() {
            continue;
        }
        if let Some(regex) = regex {
            let content = fs::read_to_string(&entry).map_err(|err| UserError::CannotReadPath {
                path: entry,
                guidance: err.to_string(),
            })?;
            return Ok(regex.is_match(&content));
        }
        return Ok(true);
    }
    Ok(false)
}
