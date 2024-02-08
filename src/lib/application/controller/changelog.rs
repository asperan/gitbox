use std::str::FromStr;

use crate::{
    application::{
        manager::message_egress_manager::MessageEgressManager,
        manager::{
            bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
            version_ingress_manager::VersionIngressManager,
        },
        options::changelog::{ChangelogOptions, FORMAT_PLACEHOLDER},
        repository_impl::{
            bounded_commit_summary_ingress_repository_impl::BoundedCommitSummaryIngressRepositoryImpl,
            semantic_version_ingress_repository_impl::SemanticVersionIngressRepositoryImpl,
        },
    },
    domain::trigger::Trigger,
    usecase::{
        configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
        usecases::{create_changelog::CreateChangelogUseCase, usecase::UseCase},
    },
};

use super::exit_code::ControllerExitCode;

pub struct ChangelogController<'a> {
    options: ChangelogOptions,
    commit_retriever: &'a dyn BoundedCommitSummaryIngressManager,
    version_retriever: &'a dyn VersionIngressManager,
    output_manager: &'a dyn MessageEgressManager,
}

impl<'a, 'b: 'a, 'c: 'a, 'd: 'a> ChangelogController<'a> {
    pub fn new(
        options: ChangelogOptions,
        commit_retriever: &'b dyn BoundedCommitSummaryIngressManager,
        version_retriever: &'c dyn VersionIngressManager,
        output_manager: &'d dyn MessageEgressManager,
    ) -> Self {
        ChangelogController {
            options,
            commit_retriever,
            version_retriever,
            output_manager,
        }
    }

    pub fn changelog(&self) -> ControllerExitCode {
        let trigger: Option<Trigger> = match self.options.exclude_trigger() {
            Some(t) => match Trigger::from_str(t) {
                Ok(v) => Some(v),
                Err(e) => {
                    self.output_manager.error(&e.to_string());
                    return ControllerExitCode::Error(1);
                }
            },
            None => None,
        };
        let configuration = ChangelogConfiguration::new(
            self.options.generate_from_latest_version(),
            ChangelogFormat::new(
                Box::new(|it| self.options.title_format().replace(FORMAT_PLACEHOLDER, it)),
                Box::new(|it| self.options.type_format().replace(FORMAT_PLACEHOLDER, it)),
                Box::new(|it| self.options.scope_format().replace(FORMAT_PLACEHOLDER, it)),
                Box::new(|it| self.options.list_format().replace(FORMAT_PLACEHOLDER, it)),
                Box::new(|it| self.options.item_format().replace(FORMAT_PLACEHOLDER, it)),
                Box::new(|it| {
                    self.options
                        .breaking_format()
                        .replace(FORMAT_PLACEHOLDER, it)
                }),
            ),
            trigger,
        );
        let bounded_commit_summary_ingress_repository_impl =
            BoundedCommitSummaryIngressRepositoryImpl::new(self.commit_retriever);
        let semantic_version_ingress_repository_impl =
            SemanticVersionIngressRepositoryImpl::new(self.version_retriever);
        let usecase = CreateChangelogUseCase::new(
            configuration,
            &bounded_commit_summary_ingress_repository_impl,
            &semantic_version_ingress_repository_impl,
        );
        match usecase.execute() {
            Ok(c) => {
                self.output_manager.output(&c);
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
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            controller::exit_code::ControllerExitCode,
            manager::message_egress_manager::MessageEgressManager,
            manager::{
                bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
                version_ingress_manager::VersionIngressManager,
            },
            options::changelog::ChangelogOptions,
        },
        domain::semantic_version::SemanticVersion,
        usecase::type_aliases::AnyError,
    };

    use super::ChangelogController;

    struct MockCommitRetriever {}
    impl BoundedCommitSummaryIngressManager for MockCommitRetriever {
        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(
                vec![
                    "feat: test".to_owned(),
                    "fix: test".to_owned(),
                    "test: test".to_owned(),
                ]
                .into_iter(),
            ))
        }
    }

    #[derive(Debug)]
    struct MockVersionError {}
    impl Display for MockVersionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MockVersionError")
        }
    }

    impl Error for MockVersionError {}

    struct MockVersionRetriever {}
    impl VersionIngressManager for MockVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Err(Box::new(MockVersionError {}))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some("0.1.0".to_owned()))
        }
    }

    struct MockOutputManager {}
    impl MessageEgressManager for MockOutputManager {
        fn output(&self, _message: &str) {}

        fn error(&self, _error: &str) {}
    }

    #[test]
    fn wrong_trigger_exits_with_error() {
        let options = ChangelogOptions::new(
            false,
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            Some("abc".to_string()),
        )
        .expect("Changelog options should be correct");
        let commit_retriever = MockCommitRetriever {};
        let version_retriever = MockVersionRetriever {};
        let output_manager = MockOutputManager {};
        let controller = ChangelogController::new(
            options,
            &commit_retriever,
            &version_retriever,
            &output_manager,
        );
        let result = controller.changelog();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }

    #[test]
    fn correct_usecase_execution() {
        let options = ChangelogOptions::new(
            false,
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            None,
        )
        .expect("Changelog options should be correct");
        let commit_retriever = MockCommitRetriever {};
        let version_retriever = MockVersionRetriever {};
        let output_manager = MockOutputManager {};
        let controller = ChangelogController::new(
            options,
            &commit_retriever,
            &version_retriever,
            &output_manager,
        );
        let result = controller.changelog();
        assert!(matches!(result, ControllerExitCode::Ok));
    }

    #[test]
    fn failed_execution_of_usecase() {
        let options = ChangelogOptions::new(
            true,
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            String::from("%s"),
            None,
        )
        .expect("Changelog options should be correct");
        let commit_retriever = MockCommitRetriever {};
        let version_retriever = MockVersionRetriever {};
        let output_manager = MockOutputManager {};
        let controller = ChangelogController::new(
            options,
            &commit_retriever,
            &version_retriever,
            &output_manager,
        );
        let result = controller.changelog();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }
}
