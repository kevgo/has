use crate::errors::GIT_NOT_INSTALLED;
use std::process::Command;

pub fn has_branch(name: &str) -> bool {
    Command::new("git")
        .args(vec!["show-ref", "--verify", &format!("refs/heads/{name}")])
        .output()
        .expect(GIT_NOT_INSTALLED)
        .status
        .success()
}
