use crate::errors::UserError;
use big_s::S;
use std::process::Command;
use std::str;

pub fn has_unpushed_commits(current_branch: &str) -> Result<bool, UserError> {
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("origin/{}..HEAD", current_branch))
        .output()
        .map_err(|_| UserError::UnknownCommand(S("git")))?;
    match str::from_utf8(&output.stdout) {
        Ok(stdout) => Ok(!stdout.trim().is_empty()),
        Err(_) => Err(UserError::GitBranchNameInvalidUnicode),
    }
}
