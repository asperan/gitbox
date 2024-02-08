use std::{rc::Rc, str::FromStr};

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

pub struct DescribeController {
    options: DescribeOptions,
    commit_summary_manager: Rc<dyn BoundedCommitSummaryIngressManager>,
    commit_metadata_manager: Rc<dyn CommitMetadataIngressManager>,
    version_manager: Rc<dyn VersionIngressManager>,
    tag_write_manager: Rc<dyn TagEgressManager>,
    output_manager: Rc<dyn MessageEgressManager>,
}

impl DescribeController {
    pub fn new(
        options: DescribeOptions,
        commit_summary_manager: Rc<dyn BoundedCommitSummaryIngressManager>,
        commit_metadata_manager: Rc<dyn CommitMetadataIngressManager>,
        version_manager: Rc<dyn VersionIngressManager>,
        tag_write_manager: Rc<dyn TagEgressManager>,
        output_manager: Rc<dyn MessageEgressManager>,
    ) -> DescribeController {
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
        let commit_summary_repository = Rc::new(BoundedCommitSummaryIngressRepositoryImpl::new(
            self.commit_summary_manager.clone(),
        ));
        let commit_metadata_repository = Rc::new(CommitMetadataIngressRepositoryImpl::new(
            self.commit_metadata_manager.clone(),
        ));
        let version_repository = Rc::new(SemanticVersionIngressRepositoryImpl::new(
            self.version_manager.clone(),
        ));
        let describe_usecase = CalculateNewVersionUseCase::new(
            describe_configuration,
            commit_summary_repository.clone(),
            commit_metadata_repository.clone(),
            version_repository,
        );
        let (new_version, old_version) = describe_usecase.execute()?;
        if self.options.diff() {
            self.output_manager.output(&format!(
                "Previous version: {}",
                old_version.map_or_else(|| String::from("None"), |it| it.to_string())
            ));
        }
        self.output_manager.output(&new_version.to_string());
        if self.options.create_tag() {
            let tag_configuration = TagConfiguration::new(
                new_version,
                self.options.tag_message().clone(),
                self.options.sign_tag(),
            )?;
            let tag_write_repository =
                Rc::new(TagEgressRepositoryImpl::new(self.tag_write_manager.clone()));
            let tag_usecase = CreateTagUseCase::new(tag_configuration, tag_write_repository);
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
            self.options.prerelease(),
            Box::new(|it| {
                self.options
                    .prerelease_pattern()
                    .replace(PRERELEASE_NUM_PLACEHOLDER, &it.to_string())
            }),
            Box::new(|it| {
                let regex = Regex::new(
                    &self
                        .options
                        .old_prerelease_pattern()
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
            self.options.prerelease_pattern() != self.options.old_prerelease_pattern(),
        );
        let metadata_configuration =
            DescribeMetadataConfiguration::new(self.options.metadata().to_vec());
        let trigger_configuration = DescribeTriggerConfiguration::new(
            Trigger::from_str(
                &self
                    .options
                    .major_trigger()
                    .clone()
                    .unwrap_or_else(|| String::from(Self::DEFAULT_MAJOR_TRIGGER_STR)),
            )?,
            Trigger::from_str(
                &self
                    .options
                    .minor_trigger()
                    .clone()
                    .unwrap_or_else(|| String::from(Self::DEFAULT_MINOR_TRIGGER_STR)),
            )?,
            Trigger::from_str(
                &self
                    .options
                    .patch_trigger()
                    .clone()
                    .unwrap_or_else(|| String::from(Self::DEFAULT_PATCH_TRIGGER_STR)),
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
    #[test]
    fn describe_controller() {
        unimplemented!();
    }
}
