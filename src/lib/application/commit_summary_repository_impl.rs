use std::{rc::Rc, str::FromStr};

use crate::{
    application::retriever::commit_retriever::CommitRetriever,
    domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
    usecases::{
        repository::bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository, type_aliases::AnyError,
    },
};

pub struct CommitSummaryRepositoryImpl {
    commit_retriever: Rc<dyn CommitRetriever>,
}

impl CommitSummaryRepositoryImpl {
    pub fn new(commit_retriever: Rc<dyn CommitRetriever>) -> CommitSummaryRepositoryImpl {
        CommitSummaryRepositoryImpl { commit_retriever }
    }
}

impl BoundedCommitSummaryIngressRepository for CommitSummaryRepositoryImpl {
    fn get_all_commits(
        &self,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
        let commit_list = self.commit_retriever.get_all_commits()?;
        Ok(Box::new(commit_list.map(|c| {
            CommitSummary::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
        let commit_list = self.commit_retriever.get_commits_from(version)?;
        Ok(Box::new(commit_list.map(|c| {
            CommitSummary::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        application::retriever::commit_retriever::CommitRetriever,
        domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
        usecases::{
            repository::bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository, type_aliases::AnyError,
        },
    };

    use super::CommitSummaryRepositoryImpl;

    struct MockCommitRetriever {}

    impl CommitRetriever for MockCommitRetriever {
        fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(vec!["test freeform", "feat: im conventional"].into_iter().map(|it| it.to_string())))
        }

        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(vec!["test freeform", "feat: im conventional"].into_iter().map(|it| it.to_string())))
        }
    }

    #[test]
    fn get_all_commits_basic() {
        let repository = CommitSummaryRepositoryImpl::new(Rc::new(MockCommitRetriever {}));
        let commit_list = repository.get_all_commits();
        assert!(commit_list.is_ok());
        assert!(commit_list
            .expect("Just asserted its OK-ness")
            .all(|it| matches!(it, CommitSummary::Conventional(..))
                || matches!(it, CommitSummary::FreeForm(..))));
    }

    #[test]
    fn get_commits_from_basic() {
        let repository = CommitSummaryRepositoryImpl::new(Rc::new(MockCommitRetriever {}));
        let commit_list = repository.get_commits_from(&None);
        assert!(commit_list.is_ok());
        assert!(commit_list
            .expect("Just asserted its OK-ness")
            .all(|it| matches!(it, CommitSummary::Conventional(..))
                || matches!(it, CommitSummary::FreeForm(..))))
    }
}
