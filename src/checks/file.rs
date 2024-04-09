use crate::errors::UserError;
use crate::fs::file_content;
use glob::{glob, Paths};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// indicates whether a file with the given name exists
pub fn exists(glob: String) -> Result<bool, UserError> {
  Ok(files_in(glob)?.next().is_some())
}

pub fn containing_text(glob: String, text: &str) -> Result<bool, UserError> {
  for entry in files_in(glob)? {
    if file_content(&entry?)?.contains(text) {
      return Ok(true);
    }
  }
  Ok(false)
}

pub fn matching_regex(glob: String, regex: String) -> Result<bool, UserError> {
  let regex = Regex::new(&regex).map_err(|err| UserError::InvalidRegex {
    pattern: regex,
    guidance: err.to_string(),
  })?;
  for entry in files_in(glob)? {
    let content = file_content(&entry?)?;
    if regex.is_match(&content) {
      return Ok(true);
    }
  }
  Ok(false)
}

/// provides an Iterator over all files matching the given glob
fn files_in(pattern: String) -> Result<FilesMatchingGlob, UserError> {
  let entries = glob(&pattern).map_err(|err| UserError::InvalidGlob {
    pattern,
    guidance: err.to_string(),
  })?;
  Ok(FilesMatchingGlob { entries })
}

/// Iterator providing all files that match the given glob
struct FilesMatchingGlob {
  entries: Paths,
}

impl Iterator for FilesMatchingGlob {
  type Item = Result<PathBuf, UserError>;

  fn next(&mut self) -> Option<Self::Item> {
    for entry in self.entries.by_ref() {
      let entry = match entry {
        Ok(entry) => entry,
        Err(err) => {
          return Some(Err(UserError::CannotReadPath {
            path: err.path().into(),
            guidance: err.to_string(),
          }));
        }
      };
      let metadata = match fs::metadata(&entry) {
        Ok(metadata) => metadata,
        Err(err) => {
          return Some(Err(UserError::CannotReadPath {
            path: entry,
            guidance: err.to_string(),
          }));
        }
      };
      if metadata.is_file() {
        return Some(Ok(entry));
      };
    }
    None
  }
}
