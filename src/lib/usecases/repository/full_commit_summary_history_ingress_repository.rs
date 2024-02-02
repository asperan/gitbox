use crate::{domain::commit_summary::CommitSummary, usecases::type_aliases::AnyError};

pub trait FullCommitSummaryHistoryIngressRepository {
    fn get_all_commits(
        &self,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError>;
}
