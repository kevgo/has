use std::process::Command;

/// detects uncommitted Git changes
pub fn uncommitted_changes() -> bool {
  let output = Command::new("git")
    .args(vec!["status", "--porcelain"])
    .output()
    .expect("git not installed");
  let output = String::from_utf8_lossy(&output.stdout);
  !output.trim().is_empty()
}
