use std::fmt::Display;

pub enum UserError {
    MissingName,
    MissingTarget,
    TooManyArguments,
    UnknownTarget(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::MissingName => f.write_str("no name provided"),
            UserError::MissingTarget => f.write_str("no target provided"),
            UserError::TooManyArguments => f.write_str("too many arguments"),
            UserError::UnknownTarget(target) => {
                f.write_fmt(format_args!("unknown target: {}", target))
            }
        }
    }
}
