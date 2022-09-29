use std::env;
use std::process;

/// the CLI arguments
pub struct Args {
    pub negate: bool,
    pub target: Target,
    pub name: String,
}

/// things to check for
pub enum Target {
    /// check for the existence of a file
    File,
}

pub fn parse(mut args: env::Args) -> Args {
    let _ = args.next(); // skip the binary name
    let mut negate: bool = false;
    let mut target_str = match args.next() {
        Some(value) => value,
        None => missing_target(),
    };
    if target_str == "no" {
        negate = true;
        target_str = match args.next() {
            Some(value) => value,
            None => missing_target(),
        };
    }
    let target = match target_str.as_str() {
        "file" => Target::File,
        other => unknown_target(other),
    };
    let name = match args.next() {
        Some(value) => value,
        None => missing_name(),
    };
    if args.next().is_some() {
        too_many_arguments();
    }
    Args {
        negate,
        target,
        name,
    }
}

fn missing_name() -> ! {
    println!("No name provided");
    process::exit(1);
}

fn missing_target() -> ! {
    println!("No target provided");
    process::exit(1);
}

fn too_many_arguments() -> ! {
    println!("Too many arguments");
    process::exit(1);
}

fn unknown_target(target: &str) -> ! {
    println!("Unknown target: {}", target);
    process::exit(1);
}
