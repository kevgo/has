use crate::cli::ContentMatch;
use crate::errors::UserError;
use glob::glob;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::fs;
use std::path::Path;

static RE: OnceCell<Result<Regex, UserError>> = OnceCell::new();

/// indicates whether a file with the given name exists
pub fn file(pattern: String, content_match: &ContentMatch) -> Result<bool, UserError> {
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
            path: entry.clone(),
            guidance: err.to_string(),
        })?;
        if !metadata.is_file() {
            continue;
        }
        match &content_match {
            ContentMatch::Regex(pattern) => {
                let regex = RE
                    .get_or_init(|| {
                        Regex::new(pattern).map_err(|err| UserError::InvalidRegex {
                            pattern: pattern.into(),
                            guidance: err.to_string(),
                        })
                    })
                    .as_ref()?;
                let content = load_content(&entry)?;
                if regex.is_match(&content) {
                    return Ok(true);
                }
            }
            ContentMatch::Text(text) => {
                let content = load_content(&entry)?;
                if content.contains(text) {
                    return Ok(true);
                }
            }
            ContentMatch::None => return Ok(true),
        };
    }
    Ok(false)
}

fn load_content(path: &Path) -> Result<String, UserError> {
    fs::read_to_string(path).map_err(|err| UserError::CannotReadPath {
        path: path.into(),
        guidance: err.to_string(),
    })
}
