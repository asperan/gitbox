use std::rc::Rc;

use crate::{
    application::manager::conventional_commit_egress_manager::ConventionalCommitEgressManager,
    domain::conventional_commit::ConventionalCommit,
    usecase::{
        repository::conventional_commit_egress_repository::ConventionalCommitEgressRepository,
        type_aliases::AnyError,
    },
};

pub struct ConventionalCommitEgressRepositoryImpl {
    conventional_commit_egress_manager: Rc<dyn ConventionalCommitEgressManager>,
}

impl ConventionalCommitEgressRepositoryImpl {
    pub fn new(
        conventional_commit_egress_manager: Rc<dyn ConventionalCommitEgressManager>,
    ) -> ConventionalCommitEgressRepositoryImpl {
        ConventionalCommitEgressRepositoryImpl {
            conventional_commit_egress_manager,
        }
    }
}

impl ConventionalCommitEgressRepository for ConventionalCommitEgressRepositoryImpl {
    fn create_commit(&self, commit: &ConventionalCommit) -> Result<(), AnyError> {
        self.conventional_commit_egress_manager
            .create_commit(&commit.to_string())
    }

    fn create_empty_commit(&self, commit: &ConventionalCommit) -> Result<(), AnyError> {
        self.conventional_commit_egress_manager
            .create_empty_commit(&commit.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            manager::conventional_commit_egress_manager::ConventionalCommitEgressManager,
            repository_impl::conventional_commit_egress_repository_impl::ConventionalCommitEgressRepositoryImpl,
        },
        domain::conventional_commit::ConventionalCommit,
        usecase::{
            repository::conventional_commit_egress_repository::ConventionalCommitEgressRepository,
            type_aliases::AnyError,
        },
    };

    #[derive(Debug)]
    struct MockError {}

    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }

    impl Error for MockError {}

    struct MockCommitManager {
        fail: bool,
    }

    impl ConventionalCommitEgressManager for MockCommitManager {
        fn create_commit(&self, _commit: &str) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
        fn create_empty_commit(&self, _commit: &str) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn create_commit_ok() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test".to_string(), None);
        let commit_manager = Rc::new(MockCommitManager { fail: false });
        let commit_repository = ConventionalCommitEgressRepositoryImpl::new(commit_manager);
        let result = commit_repository.create_commit(&commit);
        assert!(result.is_ok());
    }

    #[test]
    fn create_commit_error() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test".to_string(), None);
        let commit_manager = Rc::new(MockCommitManager { fail: true });
        let commit_repository = ConventionalCommitEgressRepositoryImpl::new(commit_manager);
        let result = commit_repository.create_commit(&commit);
        assert!(result.is_err());
    }

    #[test]
    fn create_empty_commit_ok() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test".to_string(), None);
        let commit_manager = Rc::new(MockCommitManager { fail: false });
        let commit_repository = ConventionalCommitEgressRepositoryImpl::new(commit_manager);
        let result = commit_repository.create_empty_commit(&commit);
        assert!(result.is_ok());
    }

    #[test]
    fn create_empty_commit_error() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test".to_string(), None);
        let commit_manager = Rc::new(MockCommitManager { fail: true });
        let commit_repository = ConventionalCommitEgressRepositoryImpl::new(commit_manager);
        let result = commit_repository.create_empty_commit(&commit);
        assert!(result.is_err());
    }
}
