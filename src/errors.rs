use std::fmt::Display;
use std::path::PathBuf;

pub enum UserError {
    CannotReadPath { path: PathBuf, guidance: String },
    GitBranchNameInvalidUnicode,
    InvalidGlob { pattern: String, guidance: String },
    MissingCommand,
    MissingName,
    MissingTarget,
    NonUnicodeAppOutput,
    TooManyArguments,
    UnknownCommand(String),
    UnknownTarget(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::CannotReadPath { path, guidance } => write!(
                f,
                "cannot read path \"{}\": {}",
                path.to_string_lossy(),
                guidance
            ),
            UserError::GitBranchNameInvalidUnicode => {
                f.write_str("the current Git branch name contains invalid unicode")
            }
            UserError::InvalidGlob { pattern, guidance } => {
                write!(f, "invalid glob \"{}\": {}", pattern, guidance)
            }
            UserError::UnknownCommand(cmd) => {
                write!(f, "the \"{}\" executable is not in the path", cmd)
            }
            UserError::MissingCommand => f.write_str("missing command to run"),
            UserError::MissingName => f.write_str("no name provided"),
            UserError::MissingTarget => f.write_str("no target provided"),
            UserError::NonUnicodeAppOutput => f.write_str("non-unicode application output"),
            UserError::TooManyArguments => f.write_str("too many arguments"),
            UserError::UnknownTarget(target) => write!(f, "unknown target: {}", target),
        }
    }
}
