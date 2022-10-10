use crate::errors::UserError;
use crate::git;

/// indicates the existence of a local Git branch with the given name
pub fn local(name: &str) -> bool {
    git::has_branch(name)
}

/// indicates whether the local Git workplace has the given branch checked out
pub fn local_active(name: &str) -> Result<bool, UserError> {
    Ok(git::current_branch()? == name)
}

/// indicates whether the local Git workplace has the given branch but not checked out
pub fn local_inactive(name: &str) -> Result<bool, UserError> {
    Ok(local(name) && !local_active(name)?)
}
