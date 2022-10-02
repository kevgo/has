mod checks;
mod cli;

use cli::Target;
use std::{env, process};

fn main() {
    let args = cli::parse(env::args());
    let exists = match args.target {
        Target::Branch { name } => checks::git_branch::local(&name),
        Target::File { name } => checks::file(&name),
        Target::Folder { name } => checks::folder(&name),
        Target::Help => {
            cli::help();
            process::exit(0);
        }
        Target::UncommittedChanges => checks::uncommitted_changes(),
    };
    if exists != args.should_exist {
        process::exit(1);
    }
}
