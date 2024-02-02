use crate::{
    domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
    usecases::type_aliases::AnyError,
};

pub trait CommitSummaryRepository {
    fn get_all_commits(
        &self,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError>;

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError>;
}
