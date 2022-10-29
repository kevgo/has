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
    ActiveBranch { name: String },
    InactiveBranch { name: String },
    CommandOutput { cmd: String, args: Vec<String> },
    File { name: String },
    FileWithText { name: String, content: String },
    FileWithRegex { name: String, content: String },
    Folder { name: String },
    Help,
    MakeTarget { name: String },
    NodeDependency { name: String },
    NodeDevDependency { name: String },
    UncommittedChanges,
    UnpushedChanges,
}

pub fn parse(mut args: env::Args) -> Result<Args, UserError> {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().ok_or(UserError::MissingTarget)?;
    let (should_exist, target_str) = match next.as_str() {
        "no" => (false, args.next().ok_or(UserError::MissingTarget)?),
        _ => (true, next),
    };
    let target = match target_str.as_str() {
        "active-branch" => Target::ActiveBranch {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "branch" => Target::Branch {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "inactive-branch" => Target::InactiveBranch {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "command-output" => Target::CommandOutput {
            cmd: args.next().ok_or(UserError::MissingCommand)?,
            args: args.by_ref().collect(),
        },
        "file" => {
            let name = args.next().ok_or(UserError::MissingName)?;
            match args.next() {
                Some(switch) => {
                    if !switch.starts_with("--") {
                        return Err(UserError::TooManyArguments);
                    }
                    match switch.as_str() {
                        "--containing" => Target::FileWithText {
                            name,
                            content: args.next().ok_or(UserError::MissingValueForFileContent)?,
                        },
                        "--matching" => Target::FileWithRegex {
                            name,
                            content: args.next().ok_or(UserError::MissingValueForFileContent)?,
                        },
                        _ => return Err(UserError::UnknownSwitchForFileContent { switch }),
                    }
                }
                None => Target::File { name },
            }
        }
        "folder" => Target::Folder {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "help" => Target::Help,
        "make-target" => Target::MakeTarget {
            name: args.next().ok_or(UserError::MissingMakeTarget)?,
        },
        "node-dependency" => Target::NodeDependency {
            name: args.next().ok_or(UserError::MissingNodeDependency)?,
        },
        "node-dev-dependency" => Target::NodeDevDependency {
            name: args.next().ok_or(UserError::MissingNodeDevDependency)?,
        },
        "uncommitted-changes" => Target::UncommittedChanges,
        "unpushed-commits" => Target::UnpushedChanges,
        unknown => return Err(UserError::UnknownTarget(unknown.into())),
    };
    if args.next().is_some() {
        return Err(UserError::TooManyArguments);
    }
    Ok(Args {
        should_exist,
        target,
    })
}
