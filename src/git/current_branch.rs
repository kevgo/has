use crate::errors::UserError;
use std::process::Command;

pub fn current_branch() -> Result<String, UserError> {
    let output = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output();
    let output = match output {
        Ok(output) => output,
        Err(_) => return Err(UserError::GitNotInstalled),
    };
    match String::from_utf8(output.stdout) {
        Ok(stdout) => Ok(stdout.trim().to_string()),
        Err(_) => Err(UserError::GitBranchNameInvalidUnicode),
    }
}
