use std::rc::Rc;

use crate::{
    application::{
        manager::{gitextra_write_manager::GitExtraWriteManager, output_manager::OutputManager},
        repository_impl::{
            commit_summary_repository_impl::CommitSummaryRepositoryImpl,
            gitextra_write_repository_impl::GitExtraWriteRepositoryImpl,
        },
        retriever::commit_retriever::CommitRetriever,
    },
    usecases::usecases::{
        refresh_types_and_scopes::RefreshTypesAndScopesUseCase, usecase::UseCase,
    },
};

use super::exit_code::ControllerExitCode;

pub struct RefreshController {
    commit_retriever: Rc<dyn CommitRetriever>,
    gitextra_write_manager: Rc<dyn GitExtraWriteManager>,
    output_manager: Rc<dyn OutputManager>,
}

impl RefreshController {
    pub fn new(
        commit_retriever: Rc<dyn CommitRetriever>,
        gitextra_write_manager: Rc<dyn GitExtraWriteManager>,
        output_manager: Rc<dyn OutputManager>,
    ) -> RefreshController {
        RefreshController {
            commit_retriever,
            gitextra_write_manager,
            output_manager,
        }
    }

    pub fn refresh(&self) -> ControllerExitCode {
        let usecase = RefreshTypesAndScopesUseCase::new(
            Rc::new(CommitSummaryRepositoryImpl::new(
                self.commit_retriever.clone(),
            )),
            Rc::new(GitExtraWriteRepositoryImpl::new(
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
