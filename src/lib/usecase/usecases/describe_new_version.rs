use std::rc::Rc;

use crate::{
    domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
    usecase::{
        configuration::describe::DescribeConfiguration,
        error::describe_no_relevant_changes_error::DescribeNoRelevantChangesError,
        repository::{
            bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository,
            commit_metadata_ingress_repository::CommitMetadataIngressRepository,
            semantic_version_ingress_repository::SemanticVersionIngressRepository,
        },
        type_aliases::AnyError,
    },
};

use super::usecase::UseCase;

pub struct CalculateNewVersionUseCase<'a> {
    configuration: DescribeConfiguration<'a>,
    commit_summary_repository: &'a dyn BoundedCommitSummaryIngressRepository,
    commit_metadata_repository: &'a dyn CommitMetadataIngressRepository,
    version_repository: &'a dyn SemanticVersionIngressRepository,
}

impl UseCase<(SemanticVersion, Option<SemanticVersion>)> for CalculateNewVersionUseCase<'_> {
    fn execute(&self) -> Result<(SemanticVersion, Option<SemanticVersion>), AnyError> {
        let base_version = if self.configuration.prerelease().is_active() {
            self.version_repository.last_version()
        } else {
            self.version_repository.last_stable_version()
        }?;
        let new_version = {
            let stable_version = self.next_stable(base_version.clone())?;
            let prerelease = if self.configuration.prerelease().is_active() {
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
            )? // TODO: handle error
        };
        Ok((new_version, base_version.as_ref().clone()))
    }
}

impl<'a, 'b: 'a, 'c: 'a, 'd: 'a> CalculateNewVersionUseCase<'a> {
    pub fn new(
        configuration: DescribeConfiguration<'a>,
        commit_summary_repository: &'b dyn BoundedCommitSummaryIngressRepository,
        commit_metadata_repository: &'c dyn CommitMetadataIngressRepository,
        version_repository: &'d dyn SemanticVersionIngressRepository,
    ) -> Self {
        CalculateNewVersionUseCase {
            configuration,
            commit_summary_repository,
            commit_metadata_repository,
            version_repository,
        }
    }

    #[inline]
    fn greatest_change_from(
        &self,
        version: Rc<Option<SemanticVersion>>,
    ) -> Result<Change, AnyError> {
        Ok(self
            .commit_summary_repository
            .get_commits_from(version)?
            .map(|it| self.commit_to_change(&it))
            .max()
            .unwrap_or(Change::None))
    }

    #[inline]
    fn commit_to_change(&self, commit: &CommitSummary) -> Change {
        match commit {
            CommitSummary::FreeForm(_) => Change::None,
            CommitSummary::Conventional(c) => {
                if self
                    .configuration
                    .triggers()
                    .major()
                    .accept(c.typ(), c.scope(), c.breaking())
                {
                    Change::Major
                } else if self.configuration.triggers().minor().accept(
                    c.typ(),
                    c.scope(),
                    c.breaking(),
                ) {
                    Change::Minor
                } else if self.configuration.triggers().patch().accept(
                    c.typ(),
                    c.scope().as_deref(),
                    c.breaking(),
                ) {
                    Change::Patch
                } else {
                    Change::None
                }
            }
        }
    }

    #[inline]
    fn next_stable(
        &self,
        base_version: Rc<Option<SemanticVersion>>,
    ) -> Result<StableVersion, AnyError> {
        Ok(if base_version.is_none() {
            StableVersion::first_stable()
        } else {
            let greatest_change = self.greatest_change_from(base_version.clone())?;
            let base_version = base_version
                .as_ref()
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
                    if self.configuration.prerelease().is_active() {
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
        })
    }

    #[inline]
    fn update_prerelease(&self, next_stable: &StableVersion) -> Result<String, AnyError> {
        let last_version = self.version_repository.last_version()?;
        let is_stable_updated = match last_version.as_ref() {
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
                .as_ref()
                .is_some_and(|it| it.prerelease().is_none())
        {
            Err(Box::new(DescribeNoRelevantChangesError::new()))
        } else {
            let next_prerelease_number = if self.configuration.prerelease().pattern_changed()
                || is_stable_updated
            {
                // Reset number
                1
            } else {
                let semantic_version = last_version
                    .as_ref()
                    .as_ref()
                    .expect("stable is not updated, so last version is bound to exist");
                let last_version_prerelease = semantic_version.prerelease().expect("prerelease is present, else this function would have returned an error already");
                let old_prerelease_number =
                    self.configuration.prerelease().old_pattern()(last_version_prerelease);
                old_prerelease_number + 1
            };
            Ok(self.configuration.prerelease().pattern()(
                next_prerelease_number,
            ))
        }
    }

    #[inline]
    fn generate_metadata(&self) -> Result<Option<String>, AnyError> {
        Ok(self
            .configuration
            .metadata()
            .specs()
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
            commit_summary::CommitSummary,
            conventional_commit_summary::ConventionalCommitSummary,
            semantic_version::SemanticVersion,
            trigger::{
                ArrayNode, BasicStatement, BreakingNode, InNode, ObjectNode, Start, Trigger,
                TypeNode,
            },
        },
        usecase::{
            configuration::describe::{
                DescribeConfiguration, DescribeMetadataConfiguration,
                DescribePrereleaseConfiguration, DescribeTriggerConfiguration,
            },
            error::describe_no_relevant_changes_error::DescribeNoRelevantChangesError,
            metadata_spec::MetadataSpec,
            repository::{
                bounded_commit_summary_ingress_repository::BoundedCommitSummaryIngressRepository,
                commit_metadata_ingress_repository::CommitMetadataIngressRepository,
                semantic_version_ingress_repository::SemanticVersionIngressRepository,
            },
            type_aliases::AnyError,
            usecases::{
                describe_new_version::{CalculateNewVersionUseCase, Change},
                usecase::UseCase,
            },
        },
    };

    fn trigger_configuration() -> DescribeTriggerConfiguration {
        DescribeTriggerConfiguration::new(
            Trigger::new(Start::Basic(BasicStatement::Breaking(BreakingNode {}))),
            Trigger::new(Start::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["feat".to_string()],
                },
            }))),
            Trigger::new(Start::Basic(BasicStatement::In(InNode {
                object: ObjectNode::Type(TypeNode {}),
                array: ArrayNode {
                    values: vec!["fix".to_string()],
                },
            }))),
        )
    }

    fn basic_configuration<'a>() -> DescribeConfiguration<'a> {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| it.to_string()),
            Box::new(|_it| 0),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        )
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

    impl BoundedCommitSummaryIngressRepository for MockCommitSummaryRepository {
        fn get_commits_from(
            &self,
            version: Rc<Option<SemanticVersion>>,
        ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError> {
            Ok(Box::new(
                if version
                    .as_ref()
                    .clone()
                    .is_some_and(|it| it.prerelease().is_some())
                {
                    let mut full = self.commit_list.clone();
                    full.append(self.from_prerelease.clone().as_mut());
                    full.into_iter()
                } else {
                    self.commit_list.clone().into_iter()
                },
            ))
        }
    }

    struct MockCommitMetadataRepository {}

    impl CommitMetadataIngressRepository for MockCommitMetadataRepository {
        fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError> {
            Ok(match spec {
                MetadataSpec::Sha => "sha".to_string(),
                MetadataSpec::Date => "date".to_string(),
            })
        }
    }

    struct MockVersionRepository {
        stable_version: Rc<Option<SemanticVersion>>,
        last_version: Rc<Option<SemanticVersion>>,
    }

    impl SemanticVersionIngressRepository for MockVersionRepository {
        fn last_version(&self) -> Result<Rc<Option<SemanticVersion>>, AnyError> {
            Ok(self.last_version.clone())
        }

        fn last_stable_version(&self) -> Result<Rc<Option<SemanticVersion>>, AnyError> {
            Ok(self.stable_version.clone())
        }
    }

    // test ancillary methods
    #[test]
    fn greatest_change_from_list() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "feat".to_string(),
                    None,
                    false,
                    "test".to_string(),
                ).expect("Hand-crafted commits are always correct")),
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "fix".to_string(),
                    None,
                    false,
                    "test".to_string(),
                ).expect("Hand-crafted commits are always correct")),
                CommitSummary::Conventional(ConventionalCommitSummary::new(
                    "chore".to_string(),
                    None,
                    false,
                    "test".to_string(),
                ).expect("Hand-crafted commits are always correct")),
            ],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .greatest_change_from(Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into())
            .expect(
                "greatest_change_from can only fail during commit list retrieval, which is mocked",
            );
        assert_eq!(result, Change::Minor);
    }

    #[test]
    fn greatest_change_from_empty_list() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .greatest_change_from(Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into())
            .expect(
                "greatest_change_from can only fail during commit list retrieval, which is mocked",
            );
        assert_eq!(result, Change::None);
    }

    #[test]
    fn commit_to_change_freeform() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let commit = CommitSummary::FreeForm("test freeform commit".to_string());
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::None);
    }

    #[test]
    fn commit_to_change_major() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "chore".to_string(),
            None,
            true,
            "test".to_string(),
        ).expect("Hand-crafted commits are always correct"));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Major);
    }

    #[test]
    fn commit_to_change_minor() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ).expect("Hand-crafted commits are always correct"));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Minor);
    }

    #[test]
    fn commit_to_change_patch() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "fix".to_string(),
            None,
            false,
            "test".to_string(),
        ).expect("Hand-crafted commits are always correct"));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::Patch);
    }

    #[test]
    fn commit_to_change_none() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let commit = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "chore".to_string(),
            None,
            false,
            "test".to_string(),
        ).expect("Hand-crafted commits are always correct"));
        let result = usecase.commit_to_change(&commit);
        assert_eq!(result, Change::None);
    }

    // test stable version generation
    #[test]
    fn first_stable_version_is_first_release() {
        let configuration = basic_configuration();
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct"));
    }

    #[test]
    fn first_unstable_version_is_first_release_and_first_prerelease() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|_it| 0),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: None.into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), None).expect("Hand-crafted version must be correct")
        );
    }

    #[test]
    fn patch_trigger_proc_patch_number_increase() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "fix".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version is always correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(0, 1, 1, None, None).expect("Hand-crafted version must be correct"));
    }

    #[test]
    fn minor_trigger_proc_minor_number_increase() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "feat".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(0, 2, 0, None, None).expect("Hand-crafted version must be correct"));
    }

    #[test]
    fn major_trigger_proc_major_number_increase() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                true,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version is always correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect("The first release should not have an error");
        assert_eq!(result.0, SemanticVersion::new(1, 0, 0, None, None).expect("Hand-crafted version must be correct"));
    }

    #[test]
    fn return_error_with_no_relevant_changes_when_describing_stable() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .execute()
            .expect_err("This should return an error as there are no relevant changes");
        assert!(result.is::<DescribeNoRelevantChangesError>());
    }

    // test prerelease generation
    #[test]
    fn prerelease_number_reset_on_pattern_change() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("alpha")
                    .expect("mock implementation must have a prefix 'alpha'")
                    .parse()
                    .expect("the value must be a number")
            }),
            true,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "refactor".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version is always correct")).into(),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("alpha1".to_string()),
                None,
            ).expect("Hand-crafted version must be correct"))
            .into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 1, Some("dev1".to_string()), None).expect("Hand-crafted version must be correct")
        );
    }

    #[test]
    fn prerelease_number_reset_on_stable_update() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "fix".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "feat".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version is always correct")).into(),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("dev1".to_string()),
                None,
            ).expect("Hand-crafted version must be correct"))
            .into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 2, 0, Some("dev1".to_string()), None).expect("Hand-crafted version must be correct")
        );
    }

    #[test]
    fn describe_prerelease_error_without_relevant_changes_from_stable_version() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase.execute();
        assert!(result.is_err_and(|it| it.is::<DescribeNoRelevantChangesError>()));
    }

    #[test]
    fn prerelease_number_increase() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            true,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_owned(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
            vec![CommitSummary::Conventional(ConventionalCommitSummary::new(
                "chore".to_string(),
                None,
                false,
                "test".to_string(),
            ).expect("Hand-crafted commits are always correct"))],
        );
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: Some(SemanticVersion::new(
                0,
                1,
                1,
                Some("dev1".to_string()),
                None,
            ).expect("Hand-crafted version must be correct"))
            .into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase.execute().expect("This calc should be correct");
        assert_eq!(
            result.0,
            SemanticVersion::new(0, 1, 1, Some("dev2".to_string()), None).expect("Hand-crafted version must be correct")
        );
    }

    // Test metadata generation

    #[test]
    fn empty_metadata() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, None);
    }

    #[test]
    fn single_metadata() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration = DescribeMetadataConfiguration::new(vec![MetadataSpec::Sha]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, Some("sha".to_string()));
    }

    #[test]
    fn multiple_metadata() {
        let prerelease_configuration = DescribePrereleaseConfiguration::new(
            false,
            Box::new(|it| format!("dev{}", it)),
            Box::new(|it| {
                it.strip_prefix("dev")
                    .expect("mock implementation must have a prefix 'dev'")
                    .parse()
                    .expect("the value must be a number")
            }),
            false,
        );
        let metadata_configuration =
            DescribeMetadataConfiguration::new(vec![MetadataSpec::Date, MetadataSpec::Sha]);
        let trigger_configuration = trigger_configuration();
        let configuration = DescribeConfiguration::new(
            prerelease_configuration,
            metadata_configuration,
            trigger_configuration,
        );
        let commit_summary_repository = MockCommitSummaryRepository::new(vec![], vec![]);
        let commit_metadata_repository = MockCommitMetadataRepository {};
        let version_repository = MockVersionRepository {
            stable_version: Some(SemanticVersion::new(0, 1, 0, None, None).expect("Hand-crafted version must be correct")).into(),
            last_version: None.into(),
        };
        let usecase = CalculateNewVersionUseCase::new(
            configuration,
            &commit_summary_repository,
            &commit_metadata_repository,
            &version_repository,
        );
        let result = usecase
            .generate_metadata()
            .expect("metadata generation should be correct");
        assert_eq!(result, Some("date-sha".to_string()));
    }
}
