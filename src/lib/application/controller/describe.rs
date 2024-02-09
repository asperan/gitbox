use std::str::FromStr;

use regex::Regex;

use crate::{
    application::{
        manager::{
            bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
            commit_metadata_ingress_manager::CommitMetadataIngressManager,
            version_ingress_manager::VersionIngressManager,
        },
        manager::{
            message_egress_manager::MessageEgressManager, tag_egress_manager::TagEgressManager,
        },
        options::describe::{DescribeOptions, PRERELEASE_NUM_PLACEHOLDER},
        repository_impl::{
            bounded_commit_summary_ingress_repository_impl::BoundedCommitSummaryIngressRepositoryImpl,
            commit_metadata_ingress_repository_impl::CommitMetadataIngressRepositoryImpl,
            semantic_version_ingress_repository_impl::SemanticVersionIngressRepositoryImpl,
            tag_egress_repository_impl::TagEgressRepositoryImpl,
        },
    },
    domain::trigger::Trigger,
    usecase::{
        configuration::{
            describe::{
                DescribeConfiguration, DescribeMetadataConfiguration,
                DescribePrereleaseConfiguration, DescribeTriggerConfiguration,
            },
            tag::TagConfiguration,
        },
        type_aliases::AnyError,
        usecases::{
            create_tag::CreateTagUseCase, describe_new_version::CalculateNewVersionUseCase,
            usecase::UseCase,
        },
    },
};

use super::exit_code::ControllerExitCode;

pub struct DescribeController<'a> {
    options: DescribeOptions,
    commit_summary_manager: &'a dyn BoundedCommitSummaryIngressManager,
    commit_metadata_manager: &'a dyn CommitMetadataIngressManager,
    version_manager: &'a dyn VersionIngressManager,
    tag_write_manager: &'a dyn TagEgressManager,
    output_manager: &'a dyn MessageEgressManager,
}

impl<'a, 'b: 'a, 'c: 'a, 'd: 'a, 'e: 'a, 'f: 'a> DescribeController<'a> {
    pub fn new(
        options: DescribeOptions,
        commit_summary_manager: &'b dyn BoundedCommitSummaryIngressManager,
        commit_metadata_manager: &'c dyn CommitMetadataIngressManager,
        version_manager: &'d dyn VersionIngressManager,
        tag_write_manager: &'e dyn TagEgressManager,
        output_manager: &'f dyn MessageEgressManager,
    ) -> Self {
        DescribeController {
            options,
            commit_summary_manager,
            commit_metadata_manager,
            version_manager,
            tag_write_manager,
            output_manager,
        }
    }

    pub fn describe(&self) -> ControllerExitCode {
        match self.run() {
            Ok(_) => ControllerExitCode::Ok,
            Err(e) => {
                self.output_manager.error(&e.to_string());
                ControllerExitCode::Error(1)
            }
        }
    }

    fn run(&self) -> Result<(), AnyError> {
        let describe_configuration = self.generate_describe_configuration()?;
        let commit_summary_repository =
            BoundedCommitSummaryIngressRepositoryImpl::new(self.commit_summary_manager);
        let commit_metadata_repository =
            CommitMetadataIngressRepositoryImpl::new(self.commit_metadata_manager);
        let version_repository = SemanticVersionIngressRepositoryImpl::new(self.version_manager);
        let describe_usecase = CalculateNewVersionUseCase::new(
            describe_configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let (new_version, old_version) = describe_usecase.execute()?;
        if self.options.diff() {
            self.output_manager.output(&format!(
                "Previous version: {}",
                old_version.map_or_else(|| String::from("None"), |it| it.to_string())
            ));
        }
        self.output_manager.output(&new_version.to_string());
        if self.options.tag().enabled() {
            let tag_configuration = TagConfiguration::new(
                new_version,
                self.options.tag().message().map(|it| it.to_owned()),
                self.options.tag().sign_enabled(),
            )?;
            let tag_write_repository = TagEgressRepositoryImpl::new(self.tag_write_manager);
            let tag_usecase = CreateTagUseCase::new(tag_configuration, &tag_write_repository);
            tag_usecase.execute()?;
            self.output_manager.output("Tag created successfully");
        }
        Ok(())
    }

    const DEFAULT_MAJOR_TRIGGER_STR: &'static str = "breaking";
    const DEFAULT_MINOR_TRIGGER_STR: &'static str = "type IN [ feat ]";
    const DEFAULT_PATCH_TRIGGER_STR: &'static str = "type IN [ fix ]";

    fn generate_describe_configuration(&self) -> Result<DescribeConfiguration, AnyError> {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            self.options.prerelease().enabled(),
            Box::new(|it| {
                self.options
                    .prerelease()
                    .pattern()
                    .replace(PRERELEASE_NUM_PLACEHOLDER, &it.to_string())
            }),
            Box::new(|it| {
                let regex = Regex::new(
                    &self
                        .options
                        .prerelease()
                        .old_pattern()
                        .replace(PRERELEASE_NUM_PLACEHOLDER, "(\\d+)"),
                )
                .unwrap();
                regex
                    .captures(it)
                    .expect("regex should match")
                    .get(1)
                    .expect("group 1 must be present to match")
                    .as_str()
                    .parse()
                    .unwrap()
            }),
            self.options.prerelease().pattern() != self.options.prerelease().old_pattern(),
        );
        let metadata_configuration =
            DescribeMetadataConfiguration::new(self.options.metadata().specs().to_vec());
        let trigger_configuration = DescribeTriggerConfiguration::new(
            Trigger::from_str(
                self
                    .options
                    .triggers()
                    .major()
                    .unwrap_or(Self::DEFAULT_MAJOR_TRIGGER_STR),
            )?,
            Trigger::from_str(
                self
                    .options
                    .triggers()
                    .minor()
                    .unwrap_or(Self::DEFAULT_MINOR_TRIGGER_STR),
            )?,
            Trigger::from_str(
                self
                    .options
                    .triggers()
                    .patch()
                    .unwrap_or(Self::DEFAULT_PATCH_TRIGGER_STR),
            )?,
        );
        Ok(DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        application::{
            controller::{describe::DescribeController, exit_code::ControllerExitCode},
            manager::{
                bounded_commit_summary_ingress_manager::BoundedCommitSummaryIngressManager,
                commit_metadata_ingress_manager::CommitMetadataIngressManager,
                message_egress_manager::MessageEgressManager, tag_egress_manager::TagEgressManager,
                version_ingress_manager::VersionIngressManager,
            },
            options::describe::{
                DescribeMetadataOptions, DescribeOptions, DescribePrereleaseOptions,
                DescribeTagOptions, DescribeTriggerOptions,
            },
        },
        domain::semantic_version::SemanticVersion,
        usecase::{metadata_spec::MetadataSpec, type_aliases::AnyError},
    };

    struct MockCommitSummaryManager {}
    impl BoundedCommitSummaryIngressManager for MockCommitSummaryManager {
        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            Ok(Box::new(
                vec![
                    "feat: add a feature".to_string(),
                    "test(api): add test for API".to_string(),
                    "refactor: refactor struct".to_string(),
                ]
                .into_iter(),
            ))
        }
    }

    struct MockCommitMetadataManager {}
    impl CommitMetadataIngressManager for MockCommitMetadataManager {
        fn get_metadata(&self, metadata_spec: &MetadataSpec) -> Result<String, AnyError> {
            Ok(match metadata_spec {
                MetadataSpec::Sha => "sha",
                MetadataSpec::Date => "date",
            }
            .to_string())
        }
    }

    struct MockSemanticVersionIngressManager {}
    impl VersionIngressManager for MockSemanticVersionIngressManager {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }
        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }
    }

    struct MockTagEgressManager {
        label: RefCell<Box<str>>,
    }
    impl MockTagEgressManager {
        pub fn new() -> Self {
            MockTagEgressManager {
                label: RefCell::new("".into()),
            }
        }
    }
    impl TagEgressManager for MockTagEgressManager {
        fn create_tag(
            &self,
            label: &str,
            _message: &Option<String>,
            _sign: bool,
        ) -> Result<(), AnyError> {
            self.label.replace(label.into());
            Ok(())
        }
    }

    struct MockOutputManager {
        output_buffer: RefCell<Vec<String>>,
        error_buffer: RefCell<Vec<String>>,
    }
    impl MockOutputManager {
        pub fn new() -> Self {
            MockOutputManager {
                output_buffer: RefCell::new(vec![]),
                error_buffer: RefCell::new(vec![]),
            }
        }
    }
    impl MessageEgressManager for MockOutputManager {
        fn output(&self, message: &str) {
            self.output_buffer.borrow_mut().push(message.to_string());
        }
        fn error(&self, error: &str) {
            self.error_buffer.borrow_mut().push(error.to_string());
        }
    }

    #[test]
    fn basic_usage() {
        let options = DescribeOptions::new(
            DescribePrereleaseOptions::new(false, "dev%d".to_string(), "dev%d".to_string())
                .expect("hand-crafted options are correct"),
            false,
            DescribeMetadataOptions::new(vec![]),
            DescribeTriggerOptions::new(None, None, None),
            DescribeTagOptions::new(false, None, false),
        );
        let commit_summary_manager = MockCommitSummaryManager {};
        let commit_metadata_ingress_manager = MockCommitMetadataManager {};
        let version_ingress_manager = MockSemanticVersionIngressManager {};
        let tag_egress_manager = MockTagEgressManager::new();
        let output_manager = MockOutputManager::new();
        let controller = DescribeController::new(
            options,
            &commit_summary_manager,
            &commit_metadata_ingress_manager,
            &version_ingress_manager,
            &tag_egress_manager,
            &output_manager,
        );
        let result = controller.describe();
        assert!(matches!(result, ControllerExitCode::Ok));
        assert_eq!(output_manager.output_buffer.borrow().as_ref(), ["0.1.0"]);
    }

    #[test]
    fn diff_enabled() {
        let options = DescribeOptions::new(
            DescribePrereleaseOptions::new(false, "dev%d".to_string(), "dev%d".to_string())
                .expect("hand-crafted options are correct"),
            true,
            DescribeMetadataOptions::new(vec![]),
            DescribeTriggerOptions::new(None, None, None),
            DescribeTagOptions::new(false, None, false),
        );
        let commit_summary_manager = MockCommitSummaryManager {};
        let commit_metadata_ingress_manager = MockCommitMetadataManager {};
        let version_ingress_manager = MockSemanticVersionIngressManager {};
        let tag_egress_manager = MockTagEgressManager::new();
        let output_manager = MockOutputManager::new();
        let controller = DescribeController::new(
            options,
            &commit_summary_manager,
            &commit_metadata_ingress_manager,
            &version_ingress_manager,
            &tag_egress_manager,
            &output_manager,
        );
        let result = controller.describe();
        assert!(matches!(result, ControllerExitCode::Ok));
        assert_eq!(
            output_manager.output_buffer.borrow().as_ref(),
            ["Previous version: None", "0.1.0"]
        );
    }

    #[test]
    fn tag_enabled() {
        let options = DescribeOptions::new(
            DescribePrereleaseOptions::new(false, "dev%d".to_string(), "dev%d".to_string())
                .expect("hand-crafted options are correct"),
            false,
            DescribeMetadataOptions::new(vec![]),
            DescribeTriggerOptions::new(None, None, None),
            DescribeTagOptions::new(true, None, false),
        );
        let commit_summary_manager = MockCommitSummaryManager {};
        let commit_metadata_ingress_manager = MockCommitMetadataManager {};
        let version_ingress_manager = MockSemanticVersionIngressManager {};
        let tag_egress_manager = MockTagEgressManager::new();
        let output_manager = MockOutputManager::new();
        let controller = DescribeController::new(
            options,
            &commit_summary_manager,
            &commit_metadata_ingress_manager,
            &version_ingress_manager,
            &tag_egress_manager,
            &output_manager,
        );
        let result = controller.describe();
        assert!(matches!(result, ControllerExitCode::Ok));
        assert_eq!(tag_egress_manager.label.borrow().as_ref(), "0.1.0");
    }
}
