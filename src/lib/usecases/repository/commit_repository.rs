use crate::{
    domain::{commit::Commit, semantic_version::SemanticVersion},
    usecases::type_aliases::AnyError,
};

pub trait CommitRepository {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = Commit>>, AnyError>;

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = Commit>>, AnyError>;
}
