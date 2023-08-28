use crate::errors::{UserError, GIT_NOT_INSTALLED};
use std::process::Command;
use std::str;

pub fn has_unpushed_commits(current_branch: &str) -> Result<bool, UserError> {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("origin/{current_branch}..HEAD"))
        .output()
        .expect(GIT_NOT_INSTALLED);
    match str::from_utf8(&output.stdout) {
        Ok(stdout) => Ok(!stdout.trim().is_empty()),
        Err(_) => Err(UserError::GitBranchNameInvalidUnicode),
    }
}
