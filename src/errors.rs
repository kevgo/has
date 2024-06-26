use std::fmt::Display;
use std::path::PathBuf;

pub const GIT_NOT_INSTALLED: &str = "Git not installed";

#[derive(Debug, Eq, PartialEq)]
pub enum UserError {
  CannotReadPath { path: PathBuf, guidance: String },
  GitBranchNameInvalidUnicode,
  InvalidGlob { pattern: String, guidance: String },
  InvalidRegex { pattern: String, guidance: String },
  InvalidPackageJsonStructure { guidance: String },
  MissingCommand,
  MissingMakeTarget,
  MissingName,
  MissingNodeDependency,
  MissingNodeDevDependency,
  MissingCondition,
  MissingValueForFileContent,
  NonUnicodeAppOutput,
  TooManyArguments,
  UnknownCommand(String),
  UnknownSwitchForFileContent { switch: String },
  UnknownCondition(String),
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
        write!(f, "invalid glob \"{pattern}\": {guidance}")
      }
      UserError::InvalidRegex { pattern, guidance } => {
        write!(f, "invalid regex /{pattern}/: {guidance}")
      }
      UserError::InvalidPackageJsonStructure { guidance } => {
        write!(f, "file \"package.json\" contains invalid JSON: {guidance}")
      }
      UserError::UnknownCommand(cmd) => {
        write!(f, "the \"{cmd}\" executable is not in the path")
      }
      UserError::MissingCommand => f.write_str("missing command to run"),
      UserError::MissingMakeTarget => f.write_str("missing Make target"),
      UserError::MissingName => f.write_str("no name provided"),
      UserError::MissingNodeDependency => {
        f.write_str("please provide the name of the Node dependency to look for")
      }
      UserError::MissingNodeDevDependency => {
        f.write_str("please provide the name of the Node dev-dependency to look for")
      }
      UserError::MissingValueForFileContent => {
        f.write_str("missing value for expected file content")
      }
      UserError::MissingCondition => f.write_str("no condition provided"),
      UserError::NonUnicodeAppOutput => f.write_str("non-unicode application output"),
      UserError::TooManyArguments => f.write_str("too many arguments"),
      UserError::UnknownSwitchForFileContent { switch } => {
        write!(f, "unknown switch for file check: {switch}")
      }
      UserError::UnknownCondition(name) => write!(f, "unknown condition: {name}"),
    }
  }
}

impl From<&UserError> for UserError {
  fn from(err: &UserError) -> Self {
    err.into()
  }
}
