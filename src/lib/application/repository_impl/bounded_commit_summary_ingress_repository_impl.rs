use std::{rc::Rc, str::FromStr};

use crate::{
    application::manager::bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
    domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
    usecases::{
        repository::bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct BoundedCommitSummaryIngressRepositoryImpl {
    bounded_commit_summary_ingress_manager: Rc<dyn BoundedCommitSummaryIngressManager>,
}

impl BoundedCommitSummaryIngressRepositoryImpl {
    pub fn new(
        bounded_commit_summary_ingress_manager: Rc<dyn BoundedCommitSummaryIngressManager>,
    ) -> BoundedCommitSummaryIngressRepositoryImpl {
        BoundedCommitSummaryIngressRepositoryImpl {
            bounded_commit_summary_ingress_manager,
        }
    }
}

impl BoundedCommitSummaryIngressRepository for BoundedCommitSummaryIngressRepositoryImpl {
    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
        let commit_list = self
            .bounded_commit_summary_ingress_manager
            .get_commits_from(version)?;
        Ok(Box::new(commit_list.map(|c| {
            CommitSummary::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        application::manager::bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
        domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
        usecases::{
            repository::bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository,
            type_aliases::AnyError,
        },
    };

    use super::BoundedCommitSummaryIngressRepositoryImpl;

    struct MockCommitRetriever {}

    impl BoundedCommitSummaryIngressManager for MockCommitRetriever {
        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(
                vec!["test freeform", "feat: im conventional"]
                    .into_iter()
                    .map(|it| it.to_string()),
            ))
        }
    }

    #[test]
    fn get_commits_from_basic() {
        let repository =
            BoundedCommitSummaryIngressRepositoryImpl::new(Rc::new(MockCommitRetriever {}));
        let commit_list = repository.get_commits_from(&None);
        assert!(commit_list.is_ok());
        assert!(commit_list
            .expect("Just asserted its OK-ness")
            .all(|it| matches!(it, CommitSummary::Conventional(..))
                || matches!(it, CommitSummary::FreeForm(..))))
    }
}
