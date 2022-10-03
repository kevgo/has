mod checks;
mod cli;
mod errors;

use cli::Target;
use errors::UserError;
use std::process::ExitCode;
use std::{env, process};

fn main() -> ExitCode {
    match inner() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            println!("ERROR: {}", err);
            errors::help();
            ExitCode::FAILURE
        }
    }
}

fn inner() -> Result<ExitCode, UserError> {
    let args = cli::parse(env::args())?;
    let exists = match args.target {
        Target::Branch { name } => checks::git_branch::local(&name),
        Target::File { name } => checks::file(&name),
        Target::Folder { name } => checks::folder(&name),
        Target::Help => {
            errors::help();
            process::exit(0);
        }
        Target::UncommittedChanges => checks::uncommitted_changes(),
    };
    if exists == args.should_exist {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
