use std::process::Command;

/// indicates the existence of a local Git branch with the given name
pub fn local(name: &str) -> bool {
    let output = Command::new("git")
        .args(vec![
            "show-ref",
            "--verify",
            &format!("refs/heads/{}", name),
        ])
        .output();
    output.expect("git not installed").status.success()
}
