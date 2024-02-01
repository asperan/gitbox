use crate::usecases::type_aliases::AnyError;

pub trait CommitManager {
    fn create_commit(&self, commit: &str) -> Result<(), AnyError>;

    fn create_empty_commit(&self, commit: &str) -> Result<(), AnyError>;
}
