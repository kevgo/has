use std::process::Command;

pub fn has_author(name: &str) -> bool {
    let output = Command::new("git")
        .args(vec!["log", "--pretty=format:\"%an %ae\""])
        .output()
        .expect();
    if output.status.success()

    // .status
    // .success();
}