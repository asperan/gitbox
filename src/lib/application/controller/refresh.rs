use std::rc::Rc;

use crate::{
    application::{
        manager::full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
        manager::{
            gitextra_egress_manager::GitExtraEgressManager,
            message_egress_manager::MessageEgressManager,
        },
        repository_impl::{
            full_commit_summary_history_repository_impl::FullCommitSummaryHistoryRepositoryImpl,
            gitextra_egress_repository_impl::GitExtraEgressRepositoryImpl,
        },
    },
    usecase::usecases::{refresh_types_and_scopes::RefreshTypesAndScopesUseCase, usecase::UseCase},
};

use super::exit_code::ControllerExitCode;

pub struct RefreshController {
    full_commit_summary_history_ingress_manager: Rc<dyn FullCommitSummaryHistoryIngressManager>,
    gitextra_write_manager: Rc<dyn GitExtraEgressManager>,
    output_manager: Rc<dyn MessageEgressManager>,
}

impl RefreshController {
    pub fn new(
        full_commit_summary_history_ingress_manager: Rc<dyn FullCommitSummaryHistoryIngressManager>,
        gitextra_write_manager: Rc<dyn GitExtraEgressManager>,
        output_manager: Rc<dyn MessageEgressManager>,
    ) -> RefreshController {
        RefreshController {
            full_commit_summary_history_ingress_manager,
            gitextra_write_manager,
            output_manager,
        }
    }

    pub fn refresh(&self) -> ControllerExitCode {
        let usecase = RefreshTypesAndScopesUseCase::new(
            Rc::new(FullCommitSummaryHistoryRepositoryImpl::new(
                self.full_commit_summary_history_ingress_manager.clone(),
            )),
            Rc::new(GitExtraEgressRepositoryImpl::new(
                self.gitextra_write_manager.clone(),
            )),
        );
        match usecase.execute() {
            Ok(_) => {
                self.output_manager
                    .output("Commit types and scopes refreshed");
                ControllerExitCode::Ok
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
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        application::{
            controller::{exit_code::ControllerExitCode, refresh::RefreshController},
            manager::{
                full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
                gitextra_egress_manager::GitExtraEgressManager,
                message_egress_manager::MessageEgressManager,
            },
        },
        domain::constant::DEFAULT_COMMIT_TYPES,
        usecase::type_aliases::AnyError,
    };

    struct MockFullCommitSummaryHistoryManager {}
    impl FullCommitSummaryHistoryIngressManager for MockFullCommitSummaryHistoryManager {
        fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(
                [
                    "feat(api): test".to_string(),
                    "fix(api): test".to_string(),
                    "chore(core-deps): test".to_string(),
                ]
                .into_iter(),
            ))
        }
    }

    struct MockGitExtraEgressManager {
        types: RefCell<Vec<String>>,
        scopes: RefCell<Vec<String>>,
    }
    impl MockGitExtraEgressManager {
        pub fn new() -> Self {
            MockGitExtraEgressManager {
                types: RefCell::new(vec![]),
                scopes: RefCell::new(vec![]),
            }
        }
    }

    impl GitExtraEgressManager for MockGitExtraEgressManager {
        fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            types.for_each(|it| self.types.borrow_mut().push(it));
            Ok(())
        }
        fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            scopes.for_each(|it| self.scopes.borrow_mut().push(it));
            Ok(())
        }
    }

    struct VoidMessageEgressManager {}
    impl MessageEgressManager for VoidMessageEgressManager {
        fn output(&self, _message: &str) {}
        fn error(&self, _error: &str) {}
    }

    #[test]
    fn refresh_controller() {
        let full_history_manager = Rc::new(MockFullCommitSummaryHistoryManager {});
        let git_extra_manager = Rc::new(MockGitExtraEgressManager::new());
        let void_output_manager = Rc::new(VoidMessageEgressManager {});
        let controller = RefreshController::new(
            full_history_manager.clone(),
            git_extra_manager.clone(),
            void_output_manager.clone(),
        );
        let result = controller.refresh();
        assert!(matches!(result, ControllerExitCode::Ok));
        assert_eq!(
            git_extra_manager.types.borrow().as_slice(),
            DEFAULT_COMMIT_TYPES
        );
        assert_eq!(
            git_extra_manager.scopes.borrow().as_slice(),
            &["api", "core-deps"]
        );
    }
}
