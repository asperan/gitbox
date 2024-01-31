use std::{rc::Rc, str::FromStr};

use crate::{
    application::{
        commit_repository_impl::CommitRepositoryImpl,
        manager::output_manager::OutputManager,
        options::changelog::{ChangelogOptions, FORMAT_PLACEHOLDER},
        retriever::{commit_retriever::CommitRetriever, version_retriever::VersionRetriever},
        version_repository_impl::VersionRepositoryImpl,
    },
    domain::{
        configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
        trigger::Trigger,
    },
    usecases::usecases::{create_changelog::CreateChangelogUseCase, usecase::UseCase},
};

use super::exit_code::ControllerExitCode;

pub struct ChangelogController {
    options: ChangelogOptions,
    commit_retriever: Rc<dyn CommitRetriever>,
    version_retriever: Rc<dyn VersionRetriever>,
    output_manager: Rc<dyn OutputManager>,
}

impl ChangelogController {
    pub fn new(
        options: ChangelogOptions,
        commit_retriever: Rc<dyn CommitRetriever>,
        version_retriever: Rc<dyn VersionRetriever>,
        output_manager: Rc<dyn OutputManager>,
    ) -> ChangelogController {
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
        let usecase = CreateChangelogUseCase::new(
            configuration,
            Rc::new(CommitRepositoryImpl::new(self.commit_retriever.clone())),
            Rc::new(VersionRepositoryImpl::new(self.version_retriever.clone())),
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
