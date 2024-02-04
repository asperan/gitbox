use crate::usecases::type_aliases::AnyError;

pub trait ConventionalCommitEgressManager {
    fn create_commit(&self, commit: &str) -> Result<(), AnyError>;

    fn create_empty_commit(&self, commit: &str) -> Result<(), AnyError>;
}
