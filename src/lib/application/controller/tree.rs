use std::rc::Rc;

use crate::{
    application::{
        manager::{
            git_tree_ingress_manager::GitTreeIngressManager,
            message_egress_manager::MessageEgressManager,
        },
        repository_impl::treegraphline_ingress_repository_impl::TreeGraphLineIngressRepositoryImpl,
    },
    usecase::usecases::{format_tree_graph::FormatTreeGraphUseCase, usecase::UseCase},
};

use super::exit_code::ControllerExitCode;

pub struct TreeController {
    git_tree_ingress_manager: Rc<dyn GitTreeIngressManager>,
    message_egress_manager: Rc<dyn MessageEgressManager>,
}

impl TreeController {
    pub fn new(
        git_tree_ingress_manager: Rc<dyn GitTreeIngressManager>,
        message_egress_manager: Rc<dyn MessageEgressManager>,
    ) -> Self {
        TreeController {
            git_tree_ingress_manager,
            message_egress_manager,
        }
    }

    pub fn commit_tree(&self) -> ControllerExitCode {
        let usecase = FormatTreeGraphUseCase::new(Box::new(
            TreeGraphLineIngressRepositoryImpl::new(self.git_tree_ingress_manager.clone()),
        ));
        match usecase.execute() {
            Ok(tree_graph) => {
                self.message_egress_manager.output(&tree_graph);
                ControllerExitCode::Ok
            }
            Err(e) => {
                self.message_egress_manager
                    .error(&format!("Failed to format tree graph: {}", e));
                ControllerExitCode::Error(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tree_controller() {
        unimplemented!();
    }
}
