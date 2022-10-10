mod checks;
mod cli;
mod errors;
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
        Target::EmptyOutput { cmd, args } => checks::empty_output(cmd, args)?,
        Target::File { name } => checks::file::exists(name)?,
        Target::FileWithText { name, content } => checks::file::containing_text(name, &content)?,
        Target::FileWithRegex { name, content } => checks::file::matching_regex(name, content)?,
        Target::Folder { name } => checks::folder(&name),
        Target::Help => {
            help();
            return Ok(ExitCode::SUCCESS);
        }
        Target::UncommittedChanges => checks::uncommitted_changes(),
        Target::UnpushedChanges => checks::unpushed_changes()?,
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
Usage: has [no] <target> <name>

Query files and folders:
> has [no] file <file name>
> has [no] folder <folder name>

Query Git repositories:
> has [no] branch <branch name>
> has [no] uncommitted-changes
> has [no] unpushed-commits

Query command output
> has [no] empty-output <command> [args...]

The "no" argument checks for absence of the given object.
"#
    );
}
