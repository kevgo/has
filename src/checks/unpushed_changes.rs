use std::process::Command;
use std::str;

/// detects uncommitted Git changes
pub fn unpushed_changes() -> bool {
    let current_branch = Command::new("git")
        .args(vec!["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("git not installed")
        .stdout;
    let current_branch = str::from_utf8(&current_branch).unwrap().trim();
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg(format!("origin/{}..HEAD", current_branch))
        .output()
        .expect("git not installed")
        .stdout;
    let output = str::from_utf8(&output).unwrap().trim();
    !output.is_empty()
}
