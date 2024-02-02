use std::rc::Rc;

use crate::{
    domain::{commit::CommitSummary, semantic_version::SemanticVersion},
    usecases::{
        configuration::describe::DescribeConfiguration,
        error::describe_no_relevant_changes_error::DescribeNoRelevantChangesError,
        repository::{
            commit_metadata_repository::CommitMetadataRepository,
            commit_summary_repository::CommitSummaryRepository,
            version_repository::VersionRepository,
        },
        type_aliases::AnyError,
    },
};

use super::usecase::UseCase;

pub struct CalculateNewVersionUseCase {
    configuration: DescribeConfiguration,
    commit_summary_repository: Rc<dyn CommitSummaryRepository>,
    commit_metadata_repository: Rc<dyn CommitMetadataRepository>,
    version_repository: Rc<dyn VersionRepository>,
}

impl UseCase<(SemanticVersion, Option<SemanticVersion>)> for CalculateNewVersionUseCase {
    fn execute(&self) -> Result<(SemanticVersion, Option<SemanticVersion>), AnyError> {
        let base_version = if self.configuration.prerelease() {
            self.version_repository.last_version()
        } else {
            self.version_repository.last_stable_version()
        }?;
        let new_version = {
            let stable_version = if base_version.is_none() {
                StableVersion::first_stable()
            } else {
                let greatest_change = self.greatest_change_from(&base_version)?;
                let base_version = base_version
                    .as_ref()
                    .expect("base version must be present in this branch");
                match greatest_change {
                    Change::Major => StableVersion::new(base_version.major() + 1, 0, 0),
                    Change::Minor => {
                        StableVersion::new(base_version.major(), base_version.minor() + 1, 0)
                    }
                    Change::Patch => StableVersion::new(
                        base_version.major(),
                        base_version.minor(),
                        base_version.patch() + 1,
                    ),
                    Change::None => {
                        if self.configuration.prerelease() {
                            StableVersion::new(
                                base_version.major(),
                                base_version.minor(),
                                base_version.patch(),
                            )
                        } else {
                            return Err(Box::new(DescribeNoRelevantChangesError {}));
                        }
                    }
                }
            };
            let prerelease = if self.configuration.prerelease() {
                Some(self.update_prerelease(&stable_version)?)
            } else {
                None
            };
            let metadata = self.generate_metadata()?;
            SemanticVersion::new(
                stable_version.major,
                stable_version.minor,
                stable_version.patch,
                prerelease,
                metadata,
            )
        };
        Ok((new_version, base_version))
    }
}

impl CalculateNewVersionUseCase {
    pub fn new(
        configuration: DescribeConfiguration,
        commit_summary_repository: Rc<dyn CommitSummaryRepository>,
        commit_metadata_repository: Rc<dyn CommitMetadataRepository>,
        version_repository: Rc<dyn VersionRepository>,
    ) -> CalculateNewVersionUseCase {
        CalculateNewVersionUseCase {
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        }
    }

    fn greatest_change_from(&self, version: &Option<SemanticVersion>) -> Result<Change, AnyError> {
        Ok(self
            .commit_summary_repository
            .get_commits_from(version)?
            .map(|it| self.commit_to_change(&it))
            .max()
            .unwrap_or(Change::None))
    }

    fn commit_to_change(&self, commit: &CommitSummary) -> Change {
        match commit {
            CommitSummary::FreeForm(_) => Change::None,
            CommitSummary::Conventional(c) => {
                if self
                    .configuration
                    .major_trigger()
                    .accept(c.typ(), c.scope(), c.breaking())
                {
                    Change::Major
                } else if self.configuration.minor_trigger().accept(
                    c.typ(),
                    c.scope(),
                    c.breaking(),
                ) {
                    Change::Minor
                } else if self.configuration.patch_trigger().accept(
                    c.typ(),
                    c.scope(),
                    c.breaking(),
                ) {
                    Change::Patch
                } else {
                    Change::None
                }
            }
        }
    }

    fn update_prerelease(&self, next_stable: &StableVersion) -> Result<String, AnyError> {
        let last_version = self.version_repository.last_version()?;
        let is_stable_updated = match &last_version {
            Some(old) => {
                next_stable.major != old.major()
                    || next_stable.minor != old.minor()
                    || next_stable.patch != old.patch()
            }
            None => true,
        };
        if !is_stable_updated
            && last_version
                .as_ref()
                .is_some_and(|it| it.prerelease().is_none())
        {
            Err(Box::new(DescribeNoRelevantChangesError::new()))
        } else {
            let next_prerelease_number = if self.configuration.prerelease_pattern_changed()
                || is_stable_updated
            {
                // Reset number
                1
            } else {
                let last_version_prerelease = last_version.as_ref().expect("stable is not updated, so last version is bound to exist").prerelease().as_ref().expect("prerelease is present, else this function would have returned an error already");
                let old_prerelease_number =
                    self.configuration.old_prerelease_pattern()(&last_version_prerelease);
                old_prerelease_number + 1
            };
            Ok(self.configuration.prerelease_pattern()(
                next_prerelease_number,
            ))
        }
    }

    fn generate_metadata(&self) -> Result<Option<String>, AnyError> {
        Ok(self
            .configuration
            .metadata()
            .iter()
            .map(|it| self.commit_metadata_repository.get_metadata(it))
            .collect::<Result<Vec<String>, AnyError>>()?
            .into_iter()
            .reduce(|acc, e| acc + "-" + &e))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Change {
    None,
    Patch,
    Minor,
    Major,
}

struct StableVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl StableVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> StableVersion {
        StableVersion {
            major,
            minor,
            patch,
        }
    }

    pub fn first_stable() -> StableVersion {
        StableVersion::new(0, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        domain::{
            commit::CommitSummary,
            conventional_commit_summary::ConventionalCommitSummary,
            semantic_version::SemanticVersion,
            trigger::{
                ArrayNode, BasicStatement, BreakingNode, InNode, ObjectNode, Start, Trigger,
                TypeNode,
            },
        },
        usecases::{
            configuration::describe::DescribeConfiguration,
            error::describe_no_relevant_changes_error::DescribeNoRelevantChangesError,
            metadata_spec::MetadataSpec,
            repository::{
                commit_metadata_repository::CommitMetadataRepository,
                commit_summary_repository::CommitSummaryRepository,
                version_repository::VersionRepository,
            },
            type_aliases::AnyError,
            usecases::{
                describe_new_version::{CalculateNewVersionUseCase, Change},
                usecase::UseCase,
            },
        },
    };

    fn default_major_trigger() -> Trigger {
        Trigger::new(Start::Basic(BasicStatement::Breaking(BreakingNode {})))
    }

    fn default_minor_trigger() -> Trigger {
        Trigger::new(Start::Basic(BasicStatement::In(InNode {
            object: ObjectNode::Type(TypeNode {}),
            array: ArrayNode {
                values: vec!["feat".to_string()],
            },
        })))
    }

    fn default_patch_trigger() -> Trigger {
        Trigger::new(Start::Basic(BasicStatement::In(InNode {
            object: ObjectNode::Type(TypeNode {}),
            array: ArrayNode {
                values: vec!["fix".to_string()],
            },
        })))
    }

    struct MockCommitSummaryRepository {
        commit_list: Vec<CommitSummary>,
        from_prerelease: Vec<CommitSummary>,
    }

    impl MockCommitSummaryRepository {
        pub fn new(
            commit_list: Vec<CommitSummary>,
            from_prerelease: Vec<CommitSummary>,
        ) -> MockCommitSummaryRepository {
            MockCommitSummaryRepository {
                commit_list,
                from_prerelease,
            }
        }
    }

    impl CommitSummaryRepository for MockCommitSummaryRepository {
        fn get_commits_from(
            &self,
            version: &Option<SemanticVersion>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
            Ok(Box::new(
                if version.as_ref().is_some_and(|it| it.prerelease().is_some()) {
                    let mut full = self.commit_list.clone();
                    full.append(self.from_prerelease.clone().as_mut());
                    full.into_iter()
                } else {
                    self.commit_list.clone().into_iter()
                },
            ))
        }
        fn get_all_commits(
            &self,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
            unreachable!()
        }
    }

    struct MockCommitMetadataRepository {}

    impl CommitMetadataRepository for MockCommitMetadataRepository {
        fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError> {
            Ok(match spec {
                MetadataSpec::Sha => "sha".to_string(),
                MetadataSpec::Date => "date".to_string(),
            })
        }
    }

    struct MockVersionRepository {
        stable_version: Option<SemanticVersion>,
        last_version: Option<SemanticVersion>,
    }

    impl VersionRepository for MockVersionRepository {
        fn last_version(&self) -> Result<Option<SemanticVersion>, AnyError> {
            Ok(self.last_version.clone())
        }

        fn last_stable_version(&self) -> Result<Option<SemanticVersion>, AnyError> {
            Ok(self.stable_version.clone())
        }
    }

    // test ancillary methods
    #[test]
    fn greatest_change_from_list() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "feat".to_string(),
                    None,
                    false,
                    "test".to_string(),
                )),
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "fix".to_string(),
                    None,
                    false,
                    "test".to_string(),
                )),
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "chore".to_string(),
                    None,
                    false,
                    "test".to_string(),
                )),
            ],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .greatest_change_from(&Some(SemanticVersion::first_release()))
            .expect(
                "greatest_change_from can only fail during commit list retrieval, which is mocked",
            );
        assert_eq!(result, Change::Minor);
    }

    #[test]
    fn greatest_change_from_empty_list() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .greatest_change_from(&Some(SemanticVersion::first_release()))
            .expect(
                "greatest_change_from can only fail during commit list retrieval, which is mocked",
            );
        assert_eq!(result, Change::None);
    }

    #[test]
    fn commit_to_change_freeform() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let commit = CommitSummary::FreeForm("test freeform commit".to_string());
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::None);
    }

    #[test]
    fn commit_to_change_major() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "chore".to_string(),
            None,
            true,
            "test".to_string(),
        ));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Major);
    }

    #[test]
    fn commit_to_change_minor() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Minor);
    }

    #[test]
    fn commit_to_change_patch() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "fix".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Patch);
    }

    #[test]
    fn commit_to_change_none() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "chore".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::None);
    }

    // test stable version generation
    #[test]
    fn first_stable_version_is_first_release() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::first_release());
    }

    #[test]
    fn first_unstable_version_is_first_release_and_first_prerelease() {
        let configuration = DescribeConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|_it| 0),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: None,
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), None)
        );
    }

    #[test]
    fn patch_trigger_proc_patch_number_increase() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "fix".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(0, 1, 1, None, None));
    }

    #[test]
    fn minor_trigger_proc_minor_number_increase() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "feat".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(0, 2, 0, None, None));
    }

    #[test]
    fn major_trigger_proc_major_number_increase() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                true,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(1, 0, 0, None, None));
    }

    #[test]
    fn return_error_with_no_relevant_changes_when_describing_stable() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .execute()
            .expect_err("This should return an error as there are no relevant changes");
        assert!(result.is::<DescribeNoRelevantChangesError>());
    }

    // test prerelease generation
    #[test]
    fn prerelease_number_reset_on_pattern_change() {
        let configuration = DescribeConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("alpha")
                    .expect("mock implementation must have a prefix 'alpha'")
                    .parse()
                    .expect("the value must be a number")
            }),
            true,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("alpha1".to_string()),
                None,
            )),
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 1, Some("dev1".to_string()), None)
        );
    }

    #[test]
    fn prerelease_number_reset_on_stable_update() {
        let configuration = DescribeConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "fix".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "feat".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("dev1".to_string()),
                None,
            )),
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 2, 0, Some("dev1".to_string()), None)
        );
    }

    #[test]
    fn describe_prerelease_error_without_relevant_changes_from_stable_version() {
        let configuration = DescribeConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase.execute();
        assert!(result.is_err_and(|it| it.is::<DescribeNoRelevantChangesError>()));
    }

    #[test]
    fn prerelease_number_increase() {
        let configuration = DescribeConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_owned(),
                None,
                false,
                "test".to_string(),
            ))],
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_string(),
                None,
                false,
                "test".to_string(),
            ))],
        ));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("dev1".to_string()),
                None,
            )),
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 1, Some("dev2".to_string()), None)
        );
    }

    // Test metadata generation

    #[test]
    fn empty_metadata() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, None);
    }

    #[test]
    fn single_metadata() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![MetadataSpec::Sha],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, Some("sha".to_string()));
    }

    #[test]
    fn multiple_metadata() {
        let configuration = DescribeConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
            vec![MetadataSpec::Date, MetadataSpec::Sha],
            default_major_trigger(),
            default_minor_trigger(),
            default_patch_trigger(),
        );
        let commit_summary_repository = Rc::new(MockCommitSummaryRepository::new(vec![], vec![]));
        let commit_metadata_repository = Rc::new(MockCommitMetadataRepository {});
        let version_repository = Rc::new(MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None)),
            last_version: None,
        });
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, Some("date-sha".to_string()));
    }
}
