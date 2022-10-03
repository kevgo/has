use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use std::env;
use std::process::ExitStatus;
use std::str;
use tempfile::TempDir;
use tokio::fs;
use tokio::fs::File;
use tokio::io;
use tokio::process::Command;

#[derive(Debug, World)]
pub struct HasWorld {
    dir: TempDir,
    exit_code: Option<ExitStatus>,
    output: Option<String>,
}

impl Default for HasWorld {
    fn default() -> Self {
        Self {
            dir: TempDir::new().expect("cannot create temp dir"),
            exit_code: None,
            output: None,
        }
    }
}

#[given(expr = "a file {string}")]
async fn a_file(world: &mut HasWorld, filename: String) -> io::Result<File> {
    let filepath = world.dir.path().join(filename);
    File::create(filepath).await
}

#[given(expr = "a folder {string}")]
async fn a_folder(world: &mut HasWorld, name: String) -> io::Result<()> {
    let folderpath = world.dir.path().join(name);
    fs::create_dir(folderpath).await
}

#[given(expr = "my Git workspace has a branch {string}")]
async fn has_git_branch(world: &mut HasWorld, name: String) {
    run_chk(&world.dir, "git", vec!["branch", &name]).await
}

#[given(expr = "my Git workspace is on the branch {string}")]
async fn is_on_git_branch(world: &mut HasWorld, name: String) {
    let has_branch = run_status(
        &world.dir,
        "git",
        vec![
            "show-ref",
            "--verify",
            "--quiet",
            &format!("refs/heads/{}", name),
        ],
    )
    .await;
    if has_branch {
        run_chk(&world.dir, "git", vec!["checkout", &name]).await
    } else {
        run_chk(&world.dir, "git", vec!["checkout", "-b", &name]).await
    }
}

#[given("my code is managed by Git")]
async fn git_repo(world: &mut HasWorld) {
    let dir = &world.dir;
    run_chk(dir, "git", vec!["init"]).await;
    run_chk(dir, "git", vec!["config", "user.email", "a@b.com"]).await;
    run_chk(dir, "git", vec!["config", "user.name", "Your Name"]).await;
    run_chk(dir, "git", vec!["commit", "--allow-empty", "-m", "i"]).await;
}

#[when(expr = "running {string}")]
async fn when_running(world: &mut HasWorld, command: String) {
    let mut argv = command.split_ascii_whitespace();
    match argv.next() {
        Some("has") => {}
        _ => panic!("The end-to-end tests can only run the 'has' command for now"),
    }
    let cwd = env::current_dir().expect("cannot determine current dir");
    let has_path = cwd.join("target").join("debug").join("has");
    let output = Command::new(has_path)
        .args(argv)
        .current_dir(&world.dir)
        .output()
        .await
        .expect("cannot find the 'has' executable");
    world.exit_code = Some(output.status);
    world.output = Some(String::from_utf8_lossy(&output.stdout).to_string());
}

#[then("it succeeds")]
async fn it_succeeds(world: &mut HasWorld) {
    let output = world.output.take().expect("no run recorded");
    assert_eq!(output, "");
    match world.exit_code {
        Some(have) => assert!(have.success()),
        None => panic!("no exit code registered"),
    }
}

#[then("it fails")]
async fn it_fails(world: &mut HasWorld) {
    match world.exit_code {
        Some(have) => assert!(!have.success()),
        None => panic!("no exit code registered"),
    }
}

#[then("it prints:")]
async fn it_prints(world: &mut HasWorld, step: &Step) {
    let want = step.docstring().expect("step has no docstring");
    let have = world.output.take().expect("run has first");
    assert_eq!(have.trim(), want.trim());
}

async fn run_status(dir: &TempDir, cmd: &str, args: Vec<&str>) -> bool {
    match Command::new(cmd).args(args).current_dir(dir).output().await {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

async fn run_chk(dir: &TempDir, cmd: &str, args: Vec<&str>) {
    let output = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .unwrap();
    if !output.status.success() {
        panic!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    HasWorld::run("features").await;
}
