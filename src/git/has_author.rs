use crate::errors::GIT_NOT_INSTALLED;
use std::process::Command;

pub fn has_author(name: &str) -> bool {
    let output = Command::new("git")
        .args(vec![
            "log",
            &format!("--author={name}"),
            "--pretty=format:%h",
        ])
        .output()
        .expect(GIT_NOT_INSTALLED);
    if !output.status.success() {
        return false;
    }
    output.stdout.len() > 0
}
