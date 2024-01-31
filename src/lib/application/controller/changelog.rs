use std::{rc::Rc, str::FromStr};

use crate::{
    application::{
        commit_repository_impl::CommitRepositoryImpl,
        manager::output_manager::OutputManager,
        options::changelog::{ChangelogOptions, FORMAT_PLACEHOLDER},
        retriever::{commit_retriever::CommitRetriever, version_retriever::VersionRetriever},
        version_repository_impl::VersionRepositoryImpl,
    },
    domain::trigger::Trigger,
    usecases::{
        configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
        usecases::{create_changelog::CreateChangelogUseCase, usecase::UseCase},
    },
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

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            controller::exit_code::ControllerExitCode,
            manager::output_manager::OutputManager,
            options::changelog::ChangelogOptions,
            retriever::{commit_retriever::CommitRetriever, version_retriever::VersionRetriever},
        },
        domain::{semantic_version::SemanticVersion, type_aliases::AnyError},
    };

    use super::ChangelogController;

    struct MockCommitRetriever {}
    impl CommitRetriever for MockCommitRetriever {
        fn get_all_commits(&self) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError> {
            unreachable!()
        }

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
    impl VersionRetriever for MockVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Err(Box::new(MockVersionError {}))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some("0.1.0".to_owned()))
        }
    }

    struct MockOutputManager {}
    impl OutputManager for MockOutputManager {
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
            Rc::new(commit_retriever),
            Rc::new(version_retriever),
            Rc::new(output_manager),
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
            Rc::new(commit_retriever),
            Rc::new(version_retriever),
            Rc::new(output_manager),
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
            Rc::new(commit_retriever),
            Rc::new(version_retriever),
            Rc::new(output_manager),
        );
        let result = controller.changelog();
        assert!(matches!(result, ControllerExitCode::Error(..)));
    }
}
