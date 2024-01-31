use std::{rc::Rc, str::FromStr};

use crate::{
    application::{
        commit_repository_impl::CommitRepositoryImpl,
        options::changelog::{ChangelogOptions, FORMAT_PLACEHOLDER},
        retriever::{commit_retriever::CommitRetriever, version_retriever::VersionRetriever},
        version_repository_impl::VersionRepositoryImpl,
    },
    domain::{
        configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
        trigger::Trigger,
        type_aliases::AnyError,
    },
    usecases::usecases::{create_changelog::CreateChangelogUseCase, usecase::UseCase},
};

pub struct ChangelogController {
    options: ChangelogOptions,
    commit_retriever: Rc<dyn CommitRetriever>,
    version_retriever: Rc<dyn VersionRetriever>,
}

impl ChangelogController {
    pub fn new(
        options: ChangelogOptions,
        commit_retriever: Rc<dyn CommitRetriever>,
        version_retriever: Rc<dyn VersionRetriever>,
    ) -> ChangelogController {
        ChangelogController {
            options,
            commit_retriever,
            version_retriever,
        }
    }

    pub fn changelog(&self) -> Result<String, AnyError> {
        let trigger: Option<Trigger> = match self.options.exclude_trigger() {
            Some(t) => Some(Trigger::from_str(t)?),
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
        usecase.execute()
    }
}
