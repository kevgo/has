use std::fmt::Display;

pub enum UserError {
    MissingName,
    MissingTarget,
    TooManyArguments,
    UnknownTarget,
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UserError::MissingName => "no name provided",
            UserError::MissingTarget => "no target provided",
            UserError::TooManyArguments => "too many arguments",
            UserError::UnknownTarget => "unknown target",
        })
    }
}
