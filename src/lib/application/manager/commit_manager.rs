use crate::domain::{conventional_commit::ConventionalCommit, type_aliases::AnyError};

pub trait CommitManager {
    fn create_commit(&self, commit: ConventionalCommit) -> Result<(), AnyError>;

    fn create_empty_commit(&self, commit: ConventionalCommit) -> Result<(), AnyError>;
}