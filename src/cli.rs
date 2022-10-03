use crate::errors::UserError;
use std::env;

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

pub fn parse(mut args: env::Args) -> Result<Args, UserError> {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().ok_or(UserError::MissingTarget)?;
    let (should_exist, target_str) = match next.as_str() {
        "no" => (false, args.next().ok_or(UserError::MissingTarget)?),
        _ => (true, next),
    };
    let target = match target_str.as_str() {
        "branch" => Target::Branch {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "file" => Target::File {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "folder" => Target::Folder {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "help" => Target::Help,
        "uncommitted-changes" => Target::UncommittedChanges,
        _ => return Err(UserError::UnknownTarget),
    };
    if args.next().is_some() {
        return Err(UserError::TooManyArguments);
    }
    Ok(Args {
        should_exist,
        target,
    })
}
