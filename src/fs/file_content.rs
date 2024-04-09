use crate::errors::UserError;
use std::fs;
use std::path::Path;

pub fn file_content(path: &Path) -> Result<String, UserError> {
  fs::read_to_string(path).map_err(|err| UserError::CannotReadPath {
    path: path.into(),
    guidance: err.to_string(),
  })
}
