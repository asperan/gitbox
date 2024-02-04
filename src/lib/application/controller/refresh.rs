use std::rc::Rc;

use crate::{
    application::{
        manager::{gitextra_egress_manager::GitExtraEgressManager, message_egress_manager::MessageEgressManager},
        repository_impl::{
            full_commit_summary_history_repository_impl::FullCommitSummaryHistoryRepositoryImpl,
            gitextra_egress_repository_impl::GitExtraEgressRepositoryImpl,
        },
        manager::full_commit_summary_history_ingress_manager::FullCommitSummaryHistoryIngressManager,
    },
    usecases::usecases::{
        refresh_types_and_scopes::RefreshTypesAndScopesUseCase, usecase::UseCase,
    },
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
