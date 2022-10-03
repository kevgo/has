use crate::errors::UserError;
use std::process::Command;
use std::str;

pub fn empty_output(cmd: String, args: Vec<String>) -> Result<bool, UserError> {
    let output = Command::new(&cmd)
        .args(args)
        .output()
        .map_err(|_| UserError::UnknownCommand(cmd))?;
    let text = str::from_utf8(&output.stdout).map_err(|_| UserError::NonUnicodeAppOutput)?;
    Ok(text.trim().is_empty())
}
