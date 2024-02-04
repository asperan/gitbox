use std::{rc::Rc, str::FromStr};

use crate::{
    application::retriever::full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
    domain::commit_summary::CommitSummary,
    usecases::{
        repository::full_commit_summary_history_ingress_repository::FullCommitSummaryHistoryIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct FullCommitSummaryHistoryRepositoryImpl {
    full_commit_summary_history_ingress_manager: Rc<dyn FullCommitSummaryHistoryIngressManager>,
}

impl FullCommitSummaryHistoryRepositoryImpl {
    pub fn new(
        full_commit_summary_history_ingress_manager: Rc<dyn FullCommitSummaryHistoryIngressManager>,
    ) -> FullCommitSummaryHistoryRepositoryImpl {
        FullCommitSummaryHistoryRepositoryImpl {
            full_commit_summary_history_ingress_manager,
        }
    }
}

impl FullCommitSummaryHistoryIngressRepository for FullCommitSummaryHistoryRepositoryImpl {
    fn get_all_commits(
        &self,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
        let commit_list = self
            .full_commit_summary_history_ingress_manager
            .get_all_commits()?;
        Ok(Box::new(commit_list.map(|c| {
            CommitSummary::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        application::retriever::full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
        domain::commit_summary::CommitSummary,
        usecases::{
            repository::full_commit_summary_history_ingress_repository::FullCommitSummaryHistoryIngressRepository,
            type_aliases::AnyError,
        },
    };

    use super::FullCommitSummaryHistoryRepositoryImpl;

    struct MockFullCommitSummaryHistoryIngressManager {}

    impl FullCommitSummaryHistoryIngressManager for MockFullCommitSummaryHistoryIngressManager {
        fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(
                vec!["test freeform", "feat: im conventional"]
                    .into_iter()
                    .map(|it| it.to_string()),
            ))
        }
    }

    #[test]
    fn get_all_commits_basic() {
        let repository = FullCommitSummaryHistoryRepositoryImpl::new(Rc::new(
            MockFullCommitSummaryHistoryIngressManager {},
        ));
        let commit_list = repository.get_all_commits();
        assert!(commit_list.is_ok());
        assert!(commit_list
            .expect("Just asserted its OK-ness")
            .all(|it| matches!(it, CommitSummary::Conventional(..))
                || matches!(it, CommitSummary::FreeForm(..))));
    }
}
