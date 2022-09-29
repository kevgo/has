use std::env;
use std::process;

/// the CLI arguments
pub struct Args {
    /// whether to look for presence or absence of the target
    pub should_exist: bool,
    /// the target to look for
    pub target: Target,
    /// name of the target (file or branch name)
    pub name: String,
}

/// things to check for
pub enum Target {
    /// check for the existence of a branch
    Branch,
    /// check for the existence of a file
    File,
    /// check for the existence of a folder
    Folder,
}

pub fn parse(mut args: env::Args) -> Args {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().unwrap_or_else(|| missing_target());
    let (should_exist, target_str) = match next.as_str() {
        "no" => (false, args.next().unwrap_or_else(|| missing_target())),
        _ => (true, next),
    };
    let target = match target_str.as_str() {
        "branch" => Target::Branch,
        "file" => Target::File,
        "folder" => Target::Folder,
        _ => unknown_target(&target_str),
    };
    let name = args.next().unwrap_or_else(|| missing_name());
    if args.next().is_some() {
        too_many_arguments();
    }
    Args {
        should_exist,
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
