use std::fmt::Display;

pub enum UserError {
    GitBranchNameInvalidUnicode,
    UnknownCommand(String),
    MissingCommand,
    MissingName,
    MissingTarget,
    NonUnicodeAppOutput,
    TooManyArguments,
    UnknownTarget(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::GitBranchNameInvalidUnicode => {
                f.write_str("the current Git branch name contains invalid unicode")
            }
            UserError::UnknownCommand(cmd) => f.write_fmt(format_args!(
                "the \"{}\" executable is not in the path",
                cmd
            )),
            UserError::MissingCommand => f.write_str("missing command to run"),
            UserError::MissingName => f.write_str("no name provided"),
            UserError::MissingTarget => f.write_str("no target provided"),
            UserError::NonUnicodeAppOutput => f.write_str("non-unicode application output"),
            UserError::TooManyArguments => f.write_str("too many arguments"),
            UserError::UnknownTarget(target) => {
                f.write_fmt(format_args!("unknown target: {}", target))
            }
        }
    }
}
