use crate::domain::{semantic_version::SemanticVersion, type_aliases::AnyError};

pub trait CommitRetriever {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError>;

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError>;
}
