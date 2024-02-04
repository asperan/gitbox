use std::{rc::Rc, str::FromStr};

use regex::Regex;

use crate::{
    application::{
        manager::{output_manager::OutputManager, tag_write_manager::TagWriteManager},
        options::describe::DescribeOptions,
        repository_impl::{
            commit_metadata_ingress_repository_impl::CommitMetadataIngressRepositoryImpl,
            commit_summary_repository_impl::BoundedCommitSummaryRepositoryImpl,
            tag_write_repository_impl::TagWriteRepositoryImpl,
            version_repository_impl::VersionRepositoryImpl,
        },
        retriever::{
            commit_metadata_ingress_manager::CommitMetadataIngressManager,
            commit_retriever::BoundedCommitSummaryIngressManager, version_ingress_manager::VersionIngressManager,
        },
    },
    domain::trigger::Trigger,
    usecases::{
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
    tag_write_manager: Rc<dyn TagWriteManager>,
    output_manager: Rc<dyn OutputManager>,
}

impl DescribeController {
    pub fn new(
        options: DescribeOptions,
        commit_summary_manager: Rc<dyn BoundedCommitSummaryIngressManager>,
        commit_metadata_manager: Rc<dyn CommitMetadataIngressManager>,
        version_manager: Rc<dyn VersionIngressManager>,
        tag_write_manager: Rc<dyn TagWriteManager>,
        output_manager: Rc<dyn OutputManager>,
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
        let commit_summary_repository = Rc::new(BoundedCommitSummaryRepositoryImpl::new(
            self.commit_summary_manager.clone(),
        ));
        let commit_metadata_repository = Rc::new(CommitMetadataIngressRepositoryImpl::new(
            self.commit_metadata_manager.clone(),
        ));
        let version_repository = Rc::new(VersionRepositoryImpl::new(self.version_manager.clone()));
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
                Rc::new(TagWriteRepositoryImpl::new(self.tag_write_manager.clone()));
            let tag_usecase = CreateTagUseCase::new(tag_configuration, tag_write_repository);
            tag_usecase.execute()?;
            self.output_manager.output("Tag created successfully");
        }
        Ok(())
    }

    const DEFAULT_MAJOR_TRIGGER_STR: &'static str = "breaking";
    const DEFAULT_MINOR_TRIGGER_STR: &'static str = "type IN [ feat ]";
    const DEFAULT_PATCH_TRIGGER_STR: &'static str = "type IN [ fix ]";

    fn generate_describe_configuration<'a>(
        &'a self,
    ) -> Result<DescribeConfiguration<'a>, AnyError> {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            self.options.prerelease(),
            Box::new(|it| {
                self.options
                    .prerelease_pattern()
                    .replace("%d", &it.to_string())
            }),
            Box::new(|it| {
                let regex = Regex::new(
                    &self
                        .options
                        .old_prerelease_pattern()
                        .replace("%d", "(\\d+)"),
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
