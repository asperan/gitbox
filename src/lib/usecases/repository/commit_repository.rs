use crate::domain::{commit::Commit, semantic_version::SemanticVersion};

pub trait CommitRepository {
    fn get_all_commits(&self) -> Box<dyn DoubleEndedIterator<Item = Commit>>;

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Box<dyn DoubleEndedIterator<Item = Commit>>;
}
