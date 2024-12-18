use crate::{
    application::{
        manager::{
            conventional_commit_egress_manager::ConventionalCommitEgressManager,
            init_egress_manager::InitEgressManager, message_egress_manager::MessageEgressManager,
        },
        options::init::InitOptions,
        repository_impl::conventional_commit_egress_repository_impl::ConventionalCommitEgressRepositoryImpl,
    },
    usecase::{
        configuration::commit::{AllowEmptyFlag, CommitConfiguration},
        usecases::{create_conventional_commit::CreateConventionalCommitUseCase, usecase::UseCase},
    },
};

use super::exit_code::ControllerExitCode;

pub struct InitController<'a> {
    options: InitOptions,
    init_manager: &'a dyn InitEgressManager,
    commit_manager: &'a dyn ConventionalCommitEgressManager,
    output_manager: &'a dyn MessageEgressManager,
}

impl<'a, 'b: 'a, 'c: 'a, 'd: 'a> InitController<'a> {
    pub fn new(
        options: InitOptions,
        init_manager: &'b dyn InitEgressManager,
        commit_manager: &'c dyn ConventionalCommitEgressManager,
        output_manager: &'d dyn MessageEgressManager,
    ) -> Self {
        InitController {
            options,
            init_manager,
            commit_manager,
            output_manager,
        }
    }

    pub fn init(&self) -> ControllerExitCode {
        if let Err(e) = self.init_manager.init_repository() {
            self.output_manager
                .error(&format!("Failed to init repository: {}", e));
            return ControllerExitCode::Error(1);
        }
        if !self.options.empty() {
            let configuration = CommitConfiguration::new(
                "chore".to_string(),
                Some("init".to_string()),
                false,
                "initialize empty repository".to_string(),
                None,
                AllowEmptyFlag::Enabled,
            )
            .expect("Init commit configuration is hand-made");
            let commit_repository =
                ConventionalCommitEgressRepositoryImpl::new(self.commit_manager);
            let usecase = CreateConventionalCommitUseCase::new(configuration, &commit_repository);
            if let Err(e) = usecase.execute() {
                self.output_manager.error(&e.to_string());
                return ControllerExitCode::Error(1);
            }
        }
        self.output_manager
            .output("Repository initialized successfully");
        ControllerExitCode::Ok
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display};

    use crate::{
        application::{
            controller::{exit_code::ControllerExitCode, init::InitController},
            manager::{
                conventional_commit_egress_manager::ConventionalCommitEgressManager,
                init_egress_manager::InitEgressManager,
                message_egress_manager::MessageEgressManager,
            },
            options::init::InitOptions,
        },
        usecase::type_aliases::AnyError,
    };

    #[derive(Debug)]
    struct MockError {}

    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }

    impl Error for MockError {}

    struct MockInitManager {
        fail: bool,
    }

    impl InitEgressManager for MockInitManager {
        fn init_repository(&self) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
    }

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

    struct MockOutputManager {}

    impl MessageEgressManager for MockOutputManager {
        fn output(&self, _message: &str) {}

        fn error(&self, _error: &str) {}
    }

    #[test]
    fn failed_init() {
        let options = InitOptions::new(false);
        let init_manager = MockInitManager { fail: true };
        let commit_manager = MockCommitManager { fail: true };
        let output_manager = MockOutputManager {};
        let controller =
            InitController::new(options, &init_manager, &commit_manager, &output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }

    #[test]
    fn correct_init_empty() {
        let options = InitOptions::new(true);
        let init_manager = MockInitManager { fail: false };
        let commit_manager = MockCommitManager { fail: false };
        let output_manager = MockOutputManager {};
        let controller =
            InitController::new(options, &init_manager, &commit_manager, &output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Ok));
    }

    #[test]
    fn failed_commit() {
        let options = InitOptions::new(false);
        let init_manager = MockInitManager { fail: false };
        let commit_manager = MockCommitManager { fail: true };
        let output_manager = MockOutputManager {};
        let controller =
            InitController::new(options, &init_manager, &commit_manager, &output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }

    #[test]
    fn full_init() {
        let options = InitOptions::new(false);
        let init_manager = MockInitManager { fail: false };
        let commit_manager = MockCommitManager { fail: false };
        let output_manager = MockOutputManager {};
        let controller =
            InitController::new(options, &init_manager, &commit_manager, &output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Ok));
    }
}
