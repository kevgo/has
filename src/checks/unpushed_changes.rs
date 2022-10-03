use std::process::Command;
use std::str;

/// detects uncommitted Git changes
pub fn unpushed_changes() -> bool {
    let current_branch = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("git not installed")
        .stdout;
    let output = Command::new("git")
        .arg("log")
        .arg(format!(
            "origin/{}",
            str::from_utf8(&current_branch).unwrap()
        ))
        .output()
        .expect("git not installed")
        .stdout;
    !output.is_empty()
}
