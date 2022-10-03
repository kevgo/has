use crate::errors::UserError;
use crate::git;

/// detects uncommitted Git changes
pub fn unpushed_changes() -> Result<bool, UserError> {
    git::has_unpushed_commits(&git::current_branch()?)
}
