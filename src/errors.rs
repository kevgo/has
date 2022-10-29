use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
    CannotReadPath { path: PathBuf, guidance: String },
    GitBranchNameInvalidUnicode,
    InvalidGlob { pattern: String, guidance: String },
    InvalidRegex { pattern: String, guidance: String },
    InvalidePackageJsonStructure { guidance: String },
    MissingCommand,
    MissingMakeTarget,
    MissingName,
    MissingNodeDependency,
    MissingTarget,
    MissingValueForFileContent,
    NonUnicodeAppOutput,
    TooManyArguments,
    UnknownCommand(String),
    UnknownSwitchForFileContent { switch: String },
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
            UserError::InvalidRegex { pattern, guidance } => {
                write!(f, "invalid regex /{}/: {}", pattern, guidance)
            }
            UserError::InvalidePackageJsonStructure { guidance } => {
                write!(
                    f,
                    "file \"package.json\" contains invalid JSON: {}",
                    guidance
                )
            }
            UserError::UnknownCommand(cmd) => {
                write!(f, "the \"{}\" executable is not in the path", cmd)
            }
            UserError::MissingCommand => f.write_str("missing command to run"),
            UserError::MissingMakeTarget => f.write_str("missing Make target"),
            UserError::MissingName => f.write_str("no name provided"),
            UserError::MissingNodeDependency => {
                f.write_str("please provide the name of the Node dependency to look for")
            }
            UserError::MissingValueForFileContent => {
                f.write_str("missing value for expected file content")
            }
            UserError::MissingTarget => f.write_str("no target provided"),
            UserError::NonUnicodeAppOutput => f.write_str("non-unicode application output"),
            UserError::TooManyArguments => f.write_str("too many arguments"),
            UserError::UnknownSwitchForFileContent { switch } => {
                write!(f, "unknown switch for file check: {}", switch)
            }
            UserError::UnknownTarget(target) => write!(f, "unknown target: {}", target),
        }
    }
}

impl From<&UserError> for UserError {
    fn from(err: &UserError) -> Self {
        err.into()
    }
}
