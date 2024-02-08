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
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        application::{
            controller::exit_code::ControllerExitCode,
            manager::{
                git_tree_ingress_manager::GitTreeIngressManager,
                message_egress_manager::MessageEgressManager,
            },
        },
        usecase::{tree_graph_line::TreeGraphLine, type_aliases::AnyError},
    };

    use super::TreeController;

    struct MockTreeIngressManager {}
    impl GitTreeIngressManager for MockTreeIngressManager {
        fn commit_tree(&self, _format: &str) -> Result<Box<[String]>, AnyError> {
            Ok([
                format!(
                    "* abcdef{separator}( some time ago ){separator}( HEAD -> main ){separator}",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "| {separator}{separator}{separator}asperan: first test message",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "* fedcba{separator}( some more time ago ){separator}{separator}",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "| {separator}{separator}{separator}asperan: stub test",
                    separator = TreeGraphLine::separator()
                ),
            ]
            .into())
        }
    }

    struct MockMessageEgressManager {
        output_buffer: RefCell<Vec<String>>,
        error_buffer: RefCell<Vec<String>>,
    }

    impl MockMessageEgressManager {
        pub fn new() -> Self {
            MockMessageEgressManager {
                output_buffer: RefCell::new(vec![]),
                error_buffer: RefCell::new(vec![]),
            }
        }
    }

    impl MessageEgressManager for MockMessageEgressManager {
        fn output(&self, message: &str) {
            self.output_buffer.borrow_mut().push(message.to_owned());
        }
        fn error(&self, error: &str) {
            self.error_buffer.borrow_mut().push(error.to_owned());
        }
    }

    #[test]
    fn basic_usage() {
        let tree_ingress_manager = Rc::new(MockTreeIngressManager {});
        let output_manager = Rc::new(MockMessageEgressManager::new());
        let controller = TreeController::new(tree_ingress_manager.clone(), output_manager.clone());
        let result = controller.commit_tree();
        assert!(matches!(result, ControllerExitCode::Ok));
        let expected_output = concat!(
            "       ( some time ago ) * abcdef ( HEAD -> main ) \n",
            "                         |   asperan: first test message\n",
            "  ( some more time ago ) * fedcba  \n",
            "                         |   asperan: stub test"
        );
        assert_eq!(
            output_manager
                .output_buffer
                .borrow()
                .get(0)
                .expect("The controller should have output"),
            expected_output
        );
    }
}
