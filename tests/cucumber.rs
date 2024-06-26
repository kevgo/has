use cucumber::gherkin::Step;
use cucumber::{given, then, when, World};
use shell_words::ParseError;
use std::path::Path;
use std::process::Output;
use std::{env, str};
use tempfile::TempDir;
use tokio::fs::File;
use tokio::process::Command;
use tokio::{fs, io};

#[derive(Debug, World)]
pub struct HasWorld {
  code_dir: TempDir,
  remote_dir: Option<TempDir>,
  output: Option<Output>,
}

impl Default for HasWorld {
  fn default() -> Self {
    Self {
      code_dir: TempDir::new().expect("cannot create temp dir"),
      remote_dir: None,
      output: None,
    }
  }
}

#[given(expr = "a file {string}")]
async fn a_file(world: &mut HasWorld, filename: String) -> io::Result<File> {
  let filepath = world.code_dir.path().join(filename);
  if let Some(parent) = filepath.parent() {
    fs::create_dir_all(parent).await?;
  }
  File::create(filepath).await
}

#[given(expr = "a file {string} with content:")]
async fn a_file_with_content(
  world: &mut HasWorld,
  filename: String,
  step: &Step,
) -> io::Result<()> {
  let filepath = world.code_dir.path().join(filename);
  if let Some(parent) = filepath.parent() {
    fs::create_dir_all(parent).await?;
  }
  fs::write(filepath, step.docstring().expect("no docstring")).await
}

#[given(expr = "a folder {string}")]
async fn a_folder(world: &mut HasWorld, name: String) -> io::Result<()> {
  let folderpath = world.code_dir.path().join(name);
  if let Some(parent) = folderpath.parent() {
    fs::create_dir_all(parent).await?;
  }
  fs::create_dir(folderpath).await
}

#[given("a Git repo")]
async fn git_repo(world: &mut HasWorld) {
  let dir = &world.code_dir;
  git_init(dir).await;
  run_chk(dir.path(), "git", vec!["config", "user.email", "a@b.com"]).await;
  run_chk(dir.path(), "git", vec!["config", "user.name", "Your Name"]).await;
  run_chk(
    dir.path(),
    "git",
    vec!["commit", "--allow-empty", "-m", "i"],
  )
  .await;
}

#[given(expr = "a Git repo with the user {string} and email {string}")]
async fn git_repo_with_user_and_email(world: &mut HasWorld, name: String, email: String) {
  let dir = &world.code_dir;
  git_init(dir).await;
  run_chk(dir.path(), "git", vec!["config", "user.email", &email]).await;
  run_chk(dir.path(), "git", vec!["config", "user.name", &name]).await;
  run_chk(
    dir.path(),
    "git",
    vec!["commit", "--allow-empty", "-m", "i"],
  )
  .await;
}

#[given("a local commit")]
async fn a_local_commit(world: &mut HasWorld) -> io::Result<()> {
  let filepath = &world.code_dir.path().join("committed_file");
  fs::write(filepath, b"content").await?;
  run_chk(&world.code_dir.path(), "git", vec!["add", "-A"]).await;
  run_chk(
    &world.code_dir.path(),
    "git",
    vec!["commit", "-m", "message"],
  )
  .await;
  io::Result::Ok(())
}

#[given("debug")]
async fn debug(world: &mut HasWorld) {
  println!("CODE DIR: {}", world.code_dir.path().to_string_lossy());
  let mut line = String::new();
  std::io::stdin().read_line(&mut line).unwrap();
}

#[given("my Git repo has a remote")]
async fn repo_has_git_remote(world: &mut HasWorld) {
  let remote_dir = TempDir::new().expect("cannot create temp dir");
  git_init(&remote_dir).await;
  git_add_remote(&world.code_dir, &remote_dir.path().to_string_lossy()).await;
  let current_branch = git_current_branch(&world.code_dir).await;
  git_push_branch(&world.code_dir, &current_branch).await;
  world.remote_dir = Some(remote_dir);
}

#[given(expr = "my Git workspace has a branch {string}")]
async fn create_branch(world: &mut HasWorld, name: String) {
  run_chk(&world.code_dir.path(), "git", vec!["branch", &name]).await
}

#[given(expr = "my Git workspace is on the {string} branch")]
async fn checkout_branch(world: &mut HasWorld, name: String) {
  if git_has_branch(&world.code_dir, &name).await {
    git_checkout_branch(&world.code_dir, &name).await
  } else {
    git_create_and_checkout_branch(&world.code_dir, &name).await
  }
}

#[when("running:")]
async fn when_running_docstring(world: &mut HasWorld, step: &Step) -> Result<(), ParseError> {
  let command = step.docstring().expect("no docstring").trim();
  when_running(world, command.to_string()).await
}

#[when(expr = "running {string}")]
async fn when_running(world: &mut HasWorld, command: String) -> Result<(), ParseError> {
  let mut argv = shell_words::split(&command)?.into_iter();
  if argv.next().as_deref() != Some("has") {
    panic!("The end-to-end tests can only run the 'has' command");
  }
  let cwd = env::current_dir().expect("cannot determine current dir");
  let has_path = cwd.join("target").join("debug").join("has");
  let output = Command::new(has_path)
    .args(argv)
    .current_dir(&world.code_dir)
    .output()
    .await
    .expect("cannot find the 'has' executable");
  world.output = Some(output);
  return Ok(());
}

#[then("it succeeds")]
async fn it_succeeds(world: &mut HasWorld) {
  let output = world.output.as_ref().expect("no run recorded");
  assert!(output.status.success());
}

#[then("it signals match")]
async fn it_signals_match(world: &mut HasWorld) {
  it_succeeds(world).await
}

#[then("it fails")]
async fn it_fails(world: &mut HasWorld) {
  let output = world.output.as_ref().expect("no run recorded");
  assert!(!output.status.success());
}

#[then("it signals no match")]
async fn it_signals_no_match(world: &mut HasWorld) {
  it_fails(world).await
}

#[then("it prints:")]
async fn it_prints(world: &mut HasWorld, step: &Step) {
  let want = step.docstring().expect("step has no docstring");
  let output = world.output.as_ref().expect("no run recorded");
  let have = str::from_utf8(&output.stdout).unwrap();
  pretty::assert_eq!(have.trim(), want.trim());
}

#[then("it prints nothing")]
async fn it_prints_nothing(world: &mut HasWorld) {
  let output = world.output.as_ref().expect("no run recorded");
  let have = str::from_utf8(&output.stdout).unwrap();
  pretty::assert_eq!(have.trim(), "");
}

#[then("the output starts with:")]
async fn output_contains_doc(world: &mut HasWorld, step: &Step) {
  let want = step.docstring().expect("step has no docstring");
  output_contains(world, want.to_string()).await
}

#[then(expr = "the output starts with {string}")]
async fn output_contains(world: &mut HasWorld, want: String) {
  let output = world.output.as_ref().expect("no run recorded");
  let have = str::from_utf8(&output.stdout).unwrap();
  assert!(have.trim().starts_with(want.trim()), "{}", have);
}

/// runs the given command in the given directory and returns whether it succeeded
async fn run_status(dir: &Path, cmd: &str, args: Vec<&str>) -> bool {
  match Command::new(cmd).args(args).current_dir(dir).output().await {
    io::Result::Ok(output) => output.status.success(),
    io::Result::Err(_) => false,
  }
}

/// runs the given command in the given directory and returns the STDOUT
async fn run_stdout(dir: &Path, cmd: &str, args: Vec<&str>) -> String {
  let output = Command::new(cmd)
    .args(args)
    .current_dir(dir)
    .output()
    .await
    .unwrap();
  String::from_utf8(output.stdout).unwrap()
}

/// runs the given command in the given directory and verifies it succeeds,
/// printing the output on error
async fn run_chk(dir: &Path, cmd: &str, args: Vec<&str>) {
  let output = Command::new(cmd)
    .args(args)
    .current_dir(dir)
    .output()
    .await
    .unwrap();
  if !output.status.success() {
    panic!(
      "{}{}",
      str::from_utf8(&output.stdout).unwrap(),
      str::from_utf8(&output.stderr).unwrap()
    );
  }
}

/// converts the given directory into a Git repo
async fn git_init<AP: AsRef<Path>>(dir: AP) {
  run_chk(dir.as_ref(), "git", vec!["init"]).await
}

/// adds the given remote to the Git repo in the given dir
async fn git_add_remote<AP: AsRef<Path>>(dir: AP, target: &str) {
  run_chk(dir.as_ref(), "git", vec!["remote", "add", "origin", target]).await
}

/// creates and checks out the given branch in the Git repo in the given dir
async fn git_create_and_checkout_branch<AP: AsRef<Path>>(dir: AP, branch: &str) {
  run_chk(&dir.as_ref(), "git", vec!["checkout", "-b", branch]).await
}

/// checks out the given branch in the Git repo in the given dir
async fn git_checkout_branch<AP: AsRef<Path>>(dir: AP, branch: &str) {
  run_chk(&dir.as_ref(), "git", vec!["checkout", branch]).await
}

/// provides the currently checked out branch name in the Git repo in the given dir
async fn git_current_branch<AP: AsRef<Path>>(dir: AP) -> String {
  run_stdout(
    dir.as_ref(),
    "git",
    vec!["rev-parse", "--abbrev-ref", "HEAD"],
  )
  .await
  .trim()
  .to_string()
}

/// indicates whether the Git repo in the given dir contains a branch with the given name
async fn git_has_branch<AP: AsRef<Path>>(dir: AP, branch: &str) -> bool {
  let branch_ref = &format!("refs/heads/{}", branch);
  run_status(
    dir.as_ref(),
    "git",
    vec!["show-ref", "--verify", "--quiet", branch_ref],
  )
  .await
}

/// pushes the branch with the given name in the Git repo in the given dir to the "origin" remote
async fn git_push_branch<AP: AsRef<Path>>(dir: AP, branch: &str) {
  run_chk(dir.as_ref(), "git", vec!["push", "-u", "origin", &branch]).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  HasWorld::run("features").await;
}
