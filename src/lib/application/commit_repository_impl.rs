use std::{rc::Rc, str::FromStr};

use crate::{
    application::retriever::commit_retriever::CommitRetriever,
    domain::{commit::Commit, semantic_version::SemanticVersion},
    usecases::{repository::commit_repository::CommitRepository, type_aliases::AnyError},
};

pub struct CommitRepositoryImpl {
    commit_retriever: Rc<dyn CommitRetriever>,
}

impl CommitRepositoryImpl {
    pub fn new(commit_retriever: Rc<dyn CommitRetriever>) -> CommitRepositoryImpl {
        CommitRepositoryImpl { commit_retriever }
    }
}

impl CommitRepository for CommitRepositoryImpl {
    fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = Commit>>, AnyError> {
        let commit_list = self.commit_retriever.get_all_commits()?;
        Ok(Box::new(commit_list.map(|c| {
            Commit::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }

    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = Commit>>, AnyError> {
        let commit_list = self.commit_retriever.get_commits_from(version)?;
        Ok(Box::new(commit_list.map(|c| {
            Commit::from_str(&c).expect("Commit deserialization cannot fail")
        })))
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use crate::{
        application::retriever::commit_retriever::CommitRetriever,
        domain::{commit::Commit, semantic_version::SemanticVersion},
        usecases::{repository::commit_repository::CommitRepository, type_aliases::AnyError},
    };

    use super::CommitRepositoryImpl;

    struct MockCommitRetriever {}

    impl CommitRetriever for MockCommitRetriever {
        fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(vec![""].into_iter().map(|it| it.to_string())))
        }

        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(vec![""].into_iter().map(|it| it.to_string())))
        }
    }

    #[test]
    fn get_all_commits_basic() {
        let repository = CommitRepositoryImpl::new(Rc::new(MockCommitRetriever {}));
        let commit_list = repository.get_all_commits();
        assert!(commit_list.is_ok());
        assert!(
            commit_list
                .expect("Just asserted its OK-ness")
                .all(|it| matches!(it, Commit::Conventional(..))
                    || matches!(it, Commit::FreeForm(..)))
        );
    }

    #[test]
    fn get_commits_from_basic() {
        let repository = CommitRepositoryImpl::new(Rc::new(MockCommitRetriever {}));
        let commit_list = repository.get_commits_from(&None);
        assert!(commit_list.is_ok());
        assert!(commit_list
            .expect("Just asserted its OK-ness")
            .all(|it| matches!(it, Commit::Conventional(..)) || matches!(it, Commit::FreeForm(..))))
    }
}
