use std::process::Command;
use std::str;

use crate::errors::UserError;

/// detects uncommitted Git changes
pub fn unpushed_changes() -> Result<bool, UserError> {
    let current_branch = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("git not installed")
        .stdout;
    let current_branch = match str::from_utf8(&current_branch) {
        Ok(branch) => branch.trim(),
        Err(_) => return Err(UserError::BranchNameInvalidUnicode),
    };
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("origin/{}..HEAD", current_branch))
        .output()
        .expect("git not installed")
        .stdout;
    let output = str::from_utf8(&output).unwrap().trim();
    Ok(!output.is_empty())
}
