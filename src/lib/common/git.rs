use crate::common::command_issuer::CommandIssuer;

pub fn is_in_git_repository() -> bool {
    CommandIssuer::git(vec!("rev-parse", "--is-inside-work-tree")).status.success()
}
