mod checks;
mod cli;
mod errors;
mod fs;
mod git;

use cli::Condition;
use errors::UserError;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            println!("ERROR: {err}");
            help();
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    let args = cli::parse(env::args())?;
    let exists = match args.condition {
        Condition::GitCommitsByAuthor { name } => git::has_commits_by_author(&name),
        Condition::GitBranch { name } => checks::git_branch::local(&name),
        Condition::GitBranchActive { name } => checks::git_branch::local_active(&name)?,
        Condition::GitBranchInactive { name } => checks::git_branch::local_inactive(&name)?,
        Condition::CommandOutput { cmd, args } => checks::command_output(cmd, args)?,
        Condition::File { name } => checks::file::exists(name)?,
        Condition::FileWithText { name, content } => checks::file::containing_text(name, &content)?,
        Condition::FileWithRegex { name, content } => checks::file::matching_regex(name, content)?,
        Condition::Folder { name } => checks::folder(&name),
        Condition::Help => {
            help();
            return Ok(ExitCode::SUCCESS);
        }
        Condition::MakeTarget { name } => checks::makefile::has_target(&name)?,
        Condition::NodeDependency { name } => checks::node_js::has_dependency(&name)?,
        Condition::NodeDevDependency { name } => checks::node_js::has_dev_dependency(&name)?,
        Condition::GitChangesUncommitted => checks::uncommitted_changes(),
        Condition::GitChangesUnpushed => checks::unpushed_commits()?,
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

The optional "no" argument inverts the condition.

Check filesystem:
> has [no] file <glob>
> has [no] file <glob> --containing <text>
> has [no] file <glob> --matching <regex>
> has [no] folder <glob>

Check Git:
> has [no] git-branch <branch name>
> has [no] git-branch-active <branch name>
> has [no] git-branch-inactive <branch name>
> has [no] git-changes-uncommitted
> has [no] git-commits-unpushed

Check shell commands:
> has [no] command-output <command> [args...]  # runs the given command and matches if it produces no output

Check Makefiles:
> has [no] make-target <name>

Check Node.JS codebases:
> has [no] nodejs-dependency <name>
> has [no] nodejs-dev-dependency <name>
"#
    );
}
