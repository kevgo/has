use std::fmt::Display;

pub enum UserError {
    MissingName,
    MissingTarget,
    TooManyArguments,
    UnknownTarget,
}

impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            UserError::MissingName => "no name provided",
            UserError::MissingTarget => "no target provided",
            UserError::TooManyArguments => "too many arguments",
            UserError::UnknownTarget => "unknown target",
        };
        f.write_str(text)
    }
}

pub fn help() {
    println!(
        r#"
Usage: has [no] <target> <name>

Targets define which type of object to check for:
- branch (a local Git branch)
- file
- folder
- help (print help)

Name is the name of the object to check for.

The "no" argument checks for absence of the given object.
"#
    );
}
