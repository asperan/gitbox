use crate::{
    domain::conventional_commit::ConventionalCommit,
    usecase::{
        configuration::commit::CommitConfiguration,
        error::create_conventional_commit_error::CreateConventionalCommitError,
        repository::conventional_commit_egress_repository::ConventionalCommitEgressRepository,
    },
};

use super::usecase::UseCase;

pub struct CreateConventionalCommitUseCase<'a> {
    configuration: CommitConfiguration,
    commit_repository: &'a dyn ConventionalCommitEgressRepository,
}

impl<'a, 'b: 'a> CreateConventionalCommitUseCase<'a> {
    pub fn new(
        configuration: CommitConfiguration,
        commit_repository: &'b dyn ConventionalCommitEgressRepository,
    ) -> Self {
        CreateConventionalCommitUseCase {
            configuration,
            commit_repository,
        }
    }
}

impl UseCase<ConventionalCommit, CreateConventionalCommitError>
    for CreateConventionalCommitUseCase<'_>
{
    fn execute(&self) -> Result<ConventionalCommit, CreateConventionalCommitError> {
        let commit = ConventionalCommit::new(
            self.configuration.typ().to_owned(),
            self.configuration.scope().map(|it| it.to_owned()),
            self.configuration.is_breaking().into(),
            self.configuration.summary().to_owned(),
            self.configuration.message().map(|it| it.to_owned()),
        )?;
        if self.configuration.allow_empty() {
            self.commit_repository.create_empty_commit(&commit)?;
        } else {
            self.commit_repository.create_commit(&commit)?;
        }
        Ok(commit)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display};

    use crate::{
        domain::conventional_commit::ConventionalCommit,
        usecase::{
            configuration::commit::{AllowEmptyFlag, CommitConfiguration},
            repository::conventional_commit_egress_repository::ConventionalCommitEgressRepository,
            type_aliases::AnyError,
            usecases::{
                create_conventional_commit::CreateConventionalCommitUseCase, usecase::UseCase,
            },
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

    struct MockCommitRepository {}

    impl ConventionalCommitEgressRepository for MockCommitRepository {
        fn create_commit(&self, commit: &ConventionalCommit) -> Result<(), AnyError> {
            if commit.summary().breaking() {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }

        fn create_empty_commit(&self, _commit: &ConventionalCommit) -> Result<(), AnyError> {
            unreachable!()
        }
    }

    fn simple_configuration() -> CommitConfiguration {
        CommitConfiguration::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
            None,
            AllowEmptyFlag::Disabled,
        )
        .expect("This configuration is well-formed")
    }

    fn full_configuration() -> CommitConfiguration {
        CommitConfiguration::new(
            "feat".to_string(),
            Some("test".to_string()),
            true,
            "test".to_string(),
            Some("Message body".to_string()),
            AllowEmptyFlag::Disabled,
        )
        .expect("This configuration is well-formed")
    }

    #[test]
    fn execute_usecase_correct() {
        let config = simple_configuration();
        let commit_repository = MockCommitRepository {};
        let usecase = CreateConventionalCommitUseCase::new(config, &commit_repository);
        let result = usecase.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn execute_usecase_error() {
        let config = full_configuration();
        let commit_repository = MockCommitRepository {};
        let usecase = CreateConventionalCommitUseCase::new(config, &commit_repository);
        let result = usecase.execute();
        assert!(result.is_err());
    }
}
