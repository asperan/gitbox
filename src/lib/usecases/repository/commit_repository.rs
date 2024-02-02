use crate::{domain::conventional_commit::ConventionalCommit, usecases::type_aliases::AnyError};

pub trait ConventionalCommitEgressRepository {
    fn create_commit(&self, commit: &ConventionalCommit) -> Result<(), AnyError>;
    fn create_empty_commit(&self, commit: &ConventionalCommit) -> Result<(), AnyError>;
}
