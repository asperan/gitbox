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

pub struct TreeController<'a> {
    git_tree_ingress_manager: &'a dyn GitTreeIngressManager,
    message_egress_manager: &'a dyn MessageEgressManager,
}

impl<'a: 'c, 'b: 'c, 'c> TreeController<'c> {
    pub fn new(
        git_tree_ingress_manager: &'a dyn GitTreeIngressManager,
        message_egress_manager: &'b dyn MessageEgressManager,
    ) -> Self {
        TreeController {
            git_tree_ingress_manager,
            message_egress_manager,
        }
    }

    pub fn commit_tree(&self) -> ControllerExitCode {
        let repository_impl =
            TreeGraphLineIngressRepositoryImpl::new(self.git_tree_ingress_manager);
        let usecase = FormatTreeGraphUseCase::new(&repository_impl);
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
    use std::cell::RefCell;

    use crate::{
        application::{
            controller::exit_code::ControllerExitCode,
            manager::{
                git_tree_ingress_manager::GitTreeIngressManager,
                message_egress_manager::MessageEgressManager,
            },
        },
        domain::tree_graph_line::TreeGraphLine,
        usecase::type_aliases::AnyError,
    };

    use super::TreeController;

    struct MockTreeIngressManager {}
    impl GitTreeIngressManager for MockTreeIngressManager {
        fn commit_tree(&self, _format: &str) -> Result<Box<[String]>, AnyError> {
            Ok([
                format!(
                    "* {separator}abcdef0{separator}( some time ago ){separator}( HEAD -> main ){separator}{separator}",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "| {separator}{separator}{separator}{separator}asperan: {separator}first test message",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "* {separator}0fedcba{separator}( some more time ago ){separator}{separator}{separator}",
                    separator = TreeGraphLine::separator()
                ),
                format!(
                    "| {separator}{separator}{separator}{separator}asperan: {separator}stub test",
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
        let tree_ingress_manager = MockTreeIngressManager {};
        let output_manager = MockMessageEgressManager::new();
        let controller = TreeController::new(&tree_ingress_manager, &output_manager);
        let result = controller.commit_tree();
        assert!(matches!(result, ControllerExitCode::Ok));
        let expected_output = concat!(
            "\u{1b}[2m       ( some time ago )\u{1b}[0m * \u{1b}[34mabcdef0\u{1b}[0m \u{1b}[33m( HEAD -> main )\u{1b}[0m\n",
            "                         |     \u{1b}[1;37masperan:\u{1b}[0m first test message\n",
            "\u{1b}[2m  ( some more time ago )\u{1b}[0m * \u{1b}[34m0fedcba\u{1b}[0m \u{1b}[33m\u{1b}[0m\n",
            "                         |     \u{1b}[1;37masperan:\u{1b}[0m stub test"
        );
        assert_eq!(
            output_manager
                .output_buffer
                .borrow()
                .first()
                .expect("The controller should have output"),
            expected_output
        );
    }
}
