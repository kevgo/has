mod checks;
mod cli;

use cli::Target;
use std::{env, process};

fn main() {
    let args = cli::parse(env::args());
    let found = match args.target {
        Target::File => checks::file(&args.name),
    };
    if found == args.negate {
        process::exit(1);
    }
}
