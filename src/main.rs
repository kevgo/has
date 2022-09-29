mod checks;
mod cli;

use cli::Target;
use std::{env, process};

fn main() {
    let args = cli::parse(env::args());
    let exists = match args.target {
        Target::File => checks::file(&args.name),
    };
    if exists != args.should_exist {
        process::exit(1);
    }
}
