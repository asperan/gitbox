use std::rc::Rc;

use crate::{
    domain::{commit_summary::CommitSummary, constant::DEFAULT_COMMIT_TYPES},
    usecases::{
        repository::{
            full_commit_summary_history_ingress_repository::FullCommitSummaryHistoryIngressRepository,
            git_extra_egress_repository::GitExtraEgressRepository,
        },
        type_aliases::AnyError,
    },
};

use super::usecase::UseCase;

pub struct RefreshTypesAndScopesUseCase {
    commit_history_repository: Rc<dyn FullCommitSummaryHistoryIngressRepository>,
    gitextra_write_repository: Rc<dyn GitExtraEgressRepository>,
}

impl RefreshTypesAndScopesUseCase {
    pub fn new(
        commit_history_repository: Rc<dyn FullCommitSummaryHistoryIngressRepository>,
        gitextra_write_repository: Rc<dyn GitExtraEgressRepository>,
    ) -> RefreshTypesAndScopesUseCase {
        RefreshTypesAndScopesUseCase {
            commit_history_repository,
            gitextra_write_repository,
        }
    }
}

impl UseCase<()> for RefreshTypesAndScopesUseCase {
    fn execute(&self) -> Result<(), AnyError> {
        let commits = self.commit_history_repository.get_all_commits()?;
        let mut types: Vec<String> = Vec::from(DEFAULT_COMMIT_TYPES.map(|it| it.to_string()));
        let mut scopes: Vec<String> = Vec::new();
        commits
            .filter_map(|it| match it {
                CommitSummary::Conventional(c) => Some(c),
                _ => None,
            })
            .for_each(|c| {
                if !types.contains(&c.typ().to_string()) {
                    types.push(c.typ().to_string());
                }
                if let Some(s) = c.scope() {
                    if !scopes.contains(s) {
                        scopes.push(s.to_string());
                    }
                }
            });
        self.gitextra_write_repository
            .update_types(Box::new(types.into_iter()))?;
        self.gitextra_write_repository
            .update_scopes(Box::new(scopes.into_iter()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, fmt::Display, rc::Rc};

    use crate::{
        domain::{
            commit_summary::CommitSummary, constant::DEFAULT_COMMIT_TYPES,
            conventional_commit_summary::ConventionalCommitSummary,
        },
        usecases::{
            repository::{
                full_commit_summary_history_ingress_repository::FullCommitSummaryHistoryIngressRepository,
                git_extra_egress_repository::GitExtraEgressRepository,
            },
            type_aliases::AnyError,
            usecases::usecase::UseCase,
        },
    };

    use super::RefreshTypesAndScopesUseCase;

    #[derive(Debug)]
    struct MockError {}

    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }

    impl Error for MockError {}

    struct MockFullCommitSummaryHistoryIngressRepository {}

    impl FullCommitSummaryHistoryIngressRepository for MockFullCommitSummaryHistoryIngressRepository {
        fn get_all_commits(
            &self,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
            Ok(Box::new(
                vec![
                    CommitSummary::Conventional(ConventionalCommitSummary::new(
                        "feat".to_string(),
                        Some("api".to_string()),
                        false,
                        "test".to_string(),
                    )),
                    CommitSummary::Conventional(ConventionalCommitSummary::new(
                        "feat".to_string(),
                        Some("core-deps".to_string()),
                        false,
                        "test".to_string(),
                    )),
                    CommitSummary::Conventional(ConventionalCommitSummary::new(
                        "fix".to_string(),
                        Some("core-deps".to_string()),
                        false,
                        "test".to_string(),
                    )),
                ]
                .into_iter(),
            ))
        }
    }

    struct MockGitExtraWriteRepository {
        types: RefCell<Vec<String>>,
        scopes: RefCell<Vec<String>>,
    }

    impl MockGitExtraWriteRepository {
        pub fn new() -> MockGitExtraWriteRepository {
            MockGitExtraWriteRepository {
                types: RefCell::new(Vec::new()),
                scopes: RefCell::new(Vec::new()),
            }
        }
    }

    impl GitExtraEgressRepository for MockGitExtraWriteRepository {
        fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            let _ = &self.types.replace(types.collect());
            Ok(())
        }
        fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            let _ = &self.scopes.replace(scopes.collect());
            Ok(())
        }
    }

    #[test]
    fn refresh_adds_only_distinct_values() {
        let commit_summary_repository = Rc::new(MockFullCommitSummaryHistoryIngressRepository {});
        let gitextra_write_repository = Rc::new(MockGitExtraWriteRepository::new());
        let usecase = RefreshTypesAndScopesUseCase::new(
            commit_summary_repository.clone(),
            gitextra_write_repository.clone(),
        );
        let result = usecase.execute();
        assert!(result.is_ok());
        assert_eq!(
            gitextra_write_repository.types.borrow().len(),
            DEFAULT_COMMIT_TYPES.len()
        );
        assert_eq!(gitextra_write_repository.scopes.borrow().len(), 2usize);
    }
}
