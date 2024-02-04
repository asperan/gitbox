use std::rc::Rc;

use crate::{
    application::{
        manager::{conventional_commit_egress_manager::ConventionalCommitEgressManager, message_egress_manager::MessageEgressManager},
        options::commit::CommitOptions,
        repository_impl::conventional_commit_egress_repository_impl::ConventionalCommitEgressRepositoryImpl,
    },
    usecases::{
        configuration::commit::CommitConfiguration,
        usecases::{create_conventional_commit::CreateConventionalCommitUseCase, usecase::UseCase},
    },
};

use super::exit_code::ControllerExitCode;

pub struct CommitController {
    options: CommitOptions,
    commit_manager: Rc<dyn ConventionalCommitEgressManager>,
    output_manager: Rc<dyn MessageEgressManager>,
}

impl CommitController {
    pub fn new(
        options: CommitOptions,
        commit_manager: Rc<dyn ConventionalCommitEgressManager>,
        output_manager: Rc<dyn MessageEgressManager>,
    ) -> CommitController {
        CommitController {
            options,
            commit_manager,
            output_manager,
        }
    }

    pub fn commit(&self) -> ControllerExitCode {
        match CommitConfiguration::new(
            self.options.commit_type().to_string(),
            self.options.scope().clone(),
            self.options.is_breaking(),
            self.options.summary().to_string(),
            self.options.message().clone(),
        ) {
            Ok(configuration) => {
                let commit_repository =
                    Rc::new(ConventionalCommitEgressRepositoryImpl::new(self.commit_manager.clone()));
                let usecase =
                    CreateConventionalCommitUseCase::new(configuration, commit_repository);
                match usecase.execute() {
                    Ok(c) => {
                        if !self.options.quiet() {
                            self.output_manager.output(&c.to_string());
                        }
                        ControllerExitCode::Ok
                    }
                    Err(e) => {
                        self.output_manager.error(&e.to_string());
                        ControllerExitCode::Error(1)
                    }
                }
            }
            Err(e) => {
                self.output_manager.error(&e.to_string());
                ControllerExitCode::Error(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            controller::{commit::CommitController, exit_code::ControllerExitCode},
            manager::{conventional_commit_egress_manager::ConventionalCommitEgressManager, message_egress_manager::MessageEgressManager},
            options::commit::CommitOptions,
        },
        usecases::type_aliases::AnyError,
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
        fn create_empty_commit(&self, _commit: &str) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
        fn create_commit(&self, _commit: &str) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
    }

    struct MockOutputManager {}

    impl MessageEgressManager for MockOutputManager {
        fn error(&self, _error: &str) {}
        fn output(&self, _message: &str) {}
    }

    #[test]
    fn commit_ok() {
        let options = CommitOptions::new(
            "test".to_string(),
            None,
            false,
            "test".to_string(),
            None,
            false,
        );
        let commit_manager = Rc::new(MockCommitManager { fail: false });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = CommitController::new(options, commit_manager, output_manager);
        let result = controller.commit();
        assert!(matches!(result, ControllerExitCode::Ok));
    }

    #[test]
    fn commit_error() {
        let options = CommitOptions::new(
            "test".to_string(),
            None,
            false,
            "test".to_string(),
            None,
            false,
        );
        let commit_manager = Rc::new(MockCommitManager { fail: true });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = CommitController::new(options, commit_manager, output_manager);
        let result = controller.commit();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }
}
