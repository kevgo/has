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
    Branch { name: String, state: BranchState },
    EmptyOutput { cmd: String, args: Vec<String> },
    File { name: String, content: ContentMatch },
    Folder { name: String },
    Help,
    UncommittedChanges,
    UnpushedChanges,
}

pub enum BranchState {
    Any,
    /// branch must be checked out
    Active,
    /// branch must not be checked out
    Inactive,
}

#[derive(Eq, PartialEq)]
pub enum ContentMatch {
    None,
    Text(String),
    Regex(String),
}

pub fn parse(mut args: env::Args) -> Result<Args, UserError> {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().ok_or(UserError::MissingTarget)?;
    let (should_exist, target_str) = match next.as_str() {
        "no" => (false, args.next().ok_or(UserError::MissingTarget)?),
        _ => (true, next),
    };
    let target = match target_str.as_str() {
        "active-branch" => Target::Branch {
            name: args.next().ok_or(UserError::MissingName)?,
            state: BranchState::Active,
        },
        "branch" => Target::Branch {
            name: args.next().ok_or(UserError::MissingName)?,
            state: BranchState::Any,
        },
        "inactive-branch" => Target::Branch {
            name: args.next().ok_or(UserError::MissingName)?,
            state: BranchState::Inactive,
        },
        "empty-output" => Target::EmptyOutput {
            cmd: args.next().ok_or(UserError::MissingCommand)?,
            args: args.by_ref().collect(),
        },
        "file" => {
            let name = args.next().ok_or(UserError::MissingName)?;
            let content = match args.next() {
                Some(switch) => {
                    if switch.starts_with("--") {
                        match switch.as_str() {
                            "--containing" => ContentMatch::Text(
                                args.next().ok_or(UserError::MissingValueForFileContent)?,
                            ),
                            "--matching" => ContentMatch::Regex(
                                args.next().ok_or(UserError::MissingValueForFileContent)?,
                            ),
                            _ => return Err(UserError::UnknownSwitchForFileContent { switch }),
                        }
                    } else {
                        return Err(UserError::TooManyArguments);
                    }
                }
                None => ContentMatch::None,
            };
            Target::File { name, content }
        }
        "folder" => Target::Folder {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "help" => Target::Help,
        "uncommitted-changes" => Target::UncommittedChanges,
        "unpushed-changes" => Target::UnpushedChanges,
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
