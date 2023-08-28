use crate::errors::UserError;
use std::env;

/// the CLI arguments
pub struct Args {
    /// whether to look for presence or absence of the condition
    pub should_exist: bool,
    /// the condition to check
    pub condition: Condition,
}

/// things to check for
pub enum Condition {
    GitBranch { name: String },
    GitBranchActive { name: String },
    GitBranchInactive { name: String },
    GitChangesUncommitted,
    GitChangesUnpushed,
    GitCommitsByAuthor { name: String },
    CommandOutput { cmd: String, args: Vec<String> },
    File { name: String },
    FileWithText { name: String, content: String },
    FileWithRegex { name: String, content: String },
    Folder { name: String },
    Help,
    MakeTarget { name: String },
    NodeDependency { name: String },
    NodeDevDependency { name: String },
}

pub fn parse(mut args: env::Args) -> Result<Args, UserError> {
    let _binary_name = args.next(); // skip the binary name
    let next = args.next().ok_or(UserError::MissingCondition)?;
    let (should_exist, condition) = match next.as_str() {
        "no" => (false, args.next().ok_or(UserError::MissingCondition)?),
        _ => (true, next),
    };
    let condition = match condition.as_str() {
        "command-output" => Condition::CommandOutput {
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
                        "--containing" => Condition::FileWithText {
                            name,
                            content: args.next().ok_or(UserError::MissingValueForFileContent)?,
                        },
                        "--matching" => Condition::FileWithRegex {
                            name,
                            content: args.next().ok_or(UserError::MissingValueForFileContent)?,
                        },
                        _ => return Err(UserError::UnknownSwitchForFileContent { switch }),
                    }
                }
                None => Condition::File { name },
            }
        }
        "folder" => Condition::Folder {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "git-commits-by-author" => Condition::GitCommitsByAuthor {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "git-branch" => Condition::GitBranch {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "git-branch-active" => Condition::GitBranchActive {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "git-branch-inactive" => Condition::GitBranchInactive {
            name: args.next().ok_or(UserError::MissingName)?,
        },
        "git-changes-uncommitted" => Condition::GitChangesUncommitted,
        "git-commits-unpushed" => Condition::GitChangesUnpushed,
        "help" => Condition::Help,
        "make-target" => Condition::MakeTarget {
            name: args.next().ok_or(UserError::MissingMakeTarget)?,
        },
        "nodejs-dependency" => Condition::NodeDependency {
            name: args.next().ok_or(UserError::MissingNodeDependency)?,
        },
        "nodejs-dev-dependency" => Condition::NodeDevDependency {
            name: args.next().ok_or(UserError::MissingNodeDevDependency)?,
        },
        unknown => return Err(UserError::UnknownCondition(unknown.into())),
    };
    if args.next().is_some() {
        return Err(UserError::TooManyArguments);
    }
    Ok(Args {
        should_exist,
        condition,
    })
}
