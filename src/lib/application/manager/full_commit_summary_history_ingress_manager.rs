use crate::usecase::type_aliases::AnyError;

pub trait FullCommitSummaryHistoryIngressManager {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError>;
}
