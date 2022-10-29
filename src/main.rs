mod checks;
mod cli;
mod errors;
mod fs;
mod git;

use cli::Target;
use errors::UserError;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            println!("ERROR: {}", err);
            help();
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    let args = cli::parse(env::args())?;
    let exists = match args.target {
        Target::Branch { name } => checks::git_branch::local(&name),
        Target::ActiveBranch { name } => checks::git_branch::local_active(&name)?,
        Target::InactiveBranch { name } => checks::git_branch::local_inactive(&name)?,
        Target::CommandOutput { cmd, args } => checks::command_output(cmd, args)?,
        Target::File { name } => checks::file::exists(name)?,
        Target::FileWithText { name, content } => checks::file::containing_text(name, &content)?,
        Target::FileWithRegex { name, content } => checks::file::matching_regex(name, content)?,
        Target::Folder { name } => checks::folder(&name),
        Target::Help => {
            help();
            return Ok(ExitCode::SUCCESS);
        }
        Target::MakeTarget { name } => checks::makefile::has_target(&name)?,
        Target::NodeDependency { name } => checks::node_js::has_dependency(&name)?,
        Target::UncommittedChanges => checks::uncommitted_changes(),
        Target::UnpushedChanges => checks::unpushed_commits()?,
    };
    if exists == args.should_exist {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}

fn help() {
    println!(
        r#"
Usage: has [no] <condition>

The optional "no" argument inverts the given condition.

Check for the existence of files by name and contents:
> has [no] file <glob>
> has [no] file <glob> --containing <text>
> has [no] file <glob> --matching <regex>

Check for the existence of folders:
> has [no] folder <glob>

Check for the existence and condition of Git branches:
> has [no] branch <branch name>
> has [no] active-branch <branch name>
> has [no] inactive-branch <branch name>

Check for the existence of changes that haven't been committed yet:
> has [no] uncommitted-changes

Check for the existence of commits that don't exist on the tracking branch:
> has [no] unpushed-commits

Check whether the given command produces no output:
> has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output
"#
    );
}
