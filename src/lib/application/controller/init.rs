use std::rc::Rc;

use crate::{
    application::{
        manager::{
            commit_manager::CommitManager, init_manager::InitManager, output_manager::OutputManager,
        },
        options::init::InitOptions,
    },
    domain::conventional_commit::ConventionalCommit,
};

use super::exit_code::ControllerExitCode;

pub struct InitController {
    options: InitOptions,
    init_manager: Rc<dyn InitManager>,
    commit_manager: Rc<dyn CommitManager>,
    output_manager: Rc<dyn OutputManager>,
}

impl InitController {
    pub fn new(
        options: InitOptions,
        init_manager: Rc<dyn InitManager>,
        commit_manager: Rc<dyn CommitManager>,
        output_manager: Rc<dyn OutputManager>,
    ) -> InitController {
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
                .error(&format!("Failed to init repository: {}", e.to_string()));
            return ControllerExitCode::Error(1);
        }
        if !self.options.empty() {
            let init_commit = ConventionalCommit::new(
                "chore".to_string(),
                Some("init".to_string()),
                false,
                "initial commit".to_string(),
            );
            if let Err(e) = self.commit_manager.create_empty_commit(init_commit) {
                self.output_manager.error(&format!(
                    "Failed to create initial commit: {}",
                    e.to_string()
                ));
                return ControllerExitCode::Error(1);
            }
            self.output_manager
                .output("Empty init commit created successfully");
        }
        self.output_manager
            .output("Repository initialized successfully");
        ControllerExitCode::Ok
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            controller::{exit_code::ControllerExitCode, init::InitController},
            manager::{
                commit_manager::CommitManager, init_manager::InitManager,
                output_manager::OutputManager,
            },
            options::init::InitOptions,
        },
        domain::conventional_commit::ConventionalCommit,
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

    struct MockInitManager {
        fail: bool,
    }

    impl InitManager for MockInitManager {
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

    impl CommitManager for MockCommitManager {
        fn create_commit(&self, _commit: ConventionalCommit) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }

        fn create_empty_commit(&self, _commit: ConventionalCommit) -> Result<(), AnyError> {
            if self.fail {
                Err(Box::new(MockError {}))
            } else {
                Ok(())
            }
        }
    }

    struct MockOutputManager {}

    impl OutputManager for MockOutputManager {
        fn output(&self, _message: &str) {}

        fn error(&self, _error: &str) {}
    }

    #[test]
    fn failed_init() {
        let options = InitOptions::new(false);
        let init_manager = Rc::new(MockInitManager { fail: true });
        let commit_manager = Rc::new(MockCommitManager { fail: true });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = InitController::new(options, init_manager, commit_manager, output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }

    #[test]
    fn correct_init_empty() {
        let options = InitOptions::new(true);
        let init_manager = Rc::new(MockInitManager { fail: false });
        let commit_manager = Rc::new(MockCommitManager { fail: false });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = InitController::new(options, init_manager, commit_manager, output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Ok));
    }

    #[test]
    fn failed_commit() {
        let options = InitOptions::new(false);
        let init_manager = Rc::new(MockInitManager { fail: false });
        let commit_manager = Rc::new(MockCommitManager { fail: true });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = InitController::new(options, init_manager, commit_manager, output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }

    #[test]
    fn full_init() {
        let options = InitOptions::new(false);
        let init_manager = Rc::new(MockInitManager { fail: false });
        let commit_manager = Rc::new(MockCommitManager { fail: false });
        let output_manager = Rc::new(MockOutputManager {});
        let controller = InitController::new(options, init_manager, commit_manager, output_manager);
        let result = controller.init();
        assert!(matches!(result, ControllerExitCode::Ok));
    }
}
