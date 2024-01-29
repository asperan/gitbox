use crate::domain::{commit::Commit, semantic_version::SemanticVersion};

trait CommitRetriever {
    fn get_all_commits(&self) -> impl DoubleEndedIterator<Item = Commit>;

    fn get_commits_from(&self, version: &Option<SemanticVersion>) -> impl DoubleEndedIterator<Item = Commit>;
}
