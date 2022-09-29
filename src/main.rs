mod checks;
mod cli;

use cli::Target;
use std::{env, process};

fn main() {
    let args = cli::parse(env::args());
    let exists = match args.target {
        Target::Branch => checks::git_branch::local(&args.name),
        Target::File => checks::file(&args.name),
        Target::Folder => checks::folder(&args.name),
    };
    if exists != args.should_exist {
        process::exit(1);
    }
}
