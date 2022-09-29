use cucumber::{given, then, when, World};
use std::env;
use std::process::ExitStatus;
use tempfile::TempDir;
use tokio::fs::File;
use tokio::io;
use tokio::process::Command;

#[derive(Debug, World)]
pub struct HasWorld {
    dir: TempDir,
    exit_code: Option<ExitStatus>,
}

impl Default for HasWorld {
    fn default() -> Self {
        Self {
            dir: TempDir::new().expect("cannot create temp dir"),
            exit_code: None,
        }
    }
}

#[given(expr = "a file {string}")]
async fn a_file(world: &mut HasWorld, filename: String) -> io::Result<File> {
    let filepath = world.dir.path().join(filename);
    File::create(filepath).await
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
}

#[then("it succeeds")]
fn it_returns(world: &mut HasWorld) {
    match world.exit_code {
        Some(have) => assert!(have.success()),
        None => panic!("no exit code registered"),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    HasWorld::run("features").await;
}
