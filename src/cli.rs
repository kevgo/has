use std::env;
use std::process;

/// the CLI arguments
pub struct Args {
    /// whether to look for presence or absence of the target
    pub should_exist: bool,
    /// the target to look for
    pub target: Target,
}

/// things to check for
pub enum Target {
    Branch { name: String },
    File { name: String },
    Folder { name: String },
    Help,
    UncommittedChanges,
}

pub fn parse(mut args: env::Args) -> Args {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().unwrap_or_else(|| missing_target());
    let (should_exist, target_str) = match next.as_str() {
        "no" => (false, args.next().unwrap_or_else(|| missing_target())),
        _ => (true, next),
    };
    let target = match target_str.as_str() {
        "branch" => Target::Branch {
            name: args.next().unwrap_or_else(|| missing_name()),
        },
        "file" => Target::File {
            name: args.next().unwrap_or_else(|| missing_name()),
        },
        "folder" => Target::Folder {
            name: args.next().unwrap_or_else(|| missing_name()),
        },
        "help" => Target::Help,
        "uncommitted-changes" => Target::UncommittedChanges,
        _ => unknown_target(&target_str),
    };
    if args.next().is_some() {
        too_many_arguments();
    }
    Args {
        should_exist,
        target,
    }
}

fn missing_name() -> ! {
    println!("No name provided");
    help();
    process::exit(1);
}

fn missing_target() -> ! {
    println!("No target provided");
    help();
    process::exit(1);
}

fn too_many_arguments() -> ! {
    println!("Too many arguments");
    help();
    process::exit(1);
}

fn unknown_target(target: &str) -> ! {
    println!("Unknown target: {}", target);
    help();
    process::exit(1);
}

pub fn help() {
    println!(
        r#"
Usage: has [no] <target> <name>

Targets define which type of object to check for:
- branch (a local Git branch)
- file
- folder
- help (print help)

Name is the name of the object to check for.

The "no" argument checks for absence of the given object.
"#
    );
}
