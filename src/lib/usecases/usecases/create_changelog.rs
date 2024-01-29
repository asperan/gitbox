use ahash::{AHashMap, RandomState};

use crate::{
    domain::{
        commit::Commit,
        configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
        conventional_commit::ConventionalCommit,
        semantic_version::SemanticVersion,
        trigger::Trigger,
    },
    usecases::{
        repository::{commit_repository::CommitRepository, version_repository::VersionRepository},
        usecases::usecase::UseCase,
    },
};

pub struct CreateChangelogUseCase {
    configuration: ChangelogConfiguration,
    commit_repository: Box<dyn CommitRepository>,
    version_repository: Box<dyn VersionRepository>,
}

impl CreateChangelogUseCase {
    pub fn new(
        configuration: ChangelogConfiguration,
        commit_repository: Box<dyn CommitRepository>,
        version_repository: Box<dyn VersionRepository>,
    ) -> CreateChangelogUseCase {
        CreateChangelogUseCase {
            configuration,
            commit_repository,
            version_repository,
        }
    }
}

impl UseCase<String> for CreateChangelogUseCase {
    fn execute(&self) -> String {
        let from_version = if self.configuration.generate_from_latest_version() {
            self.version_repository.last_version()
        } else {
            self.version_repository.last_stable_version()
        };
        let commit_list = self.commit_repository.get_commits_from(&from_version);

        let type_map = categorize_commit_list(commit_list, self.configuration.exclude_trigger());
        let text = format_types(self.configuration.format(), &type_map);
        let title = format_title(self.configuration.format(), &from_version);
        format!("{}\n{}", title, text)
    }
}

fn format_title(format: &ChangelogFormat, version: &Option<SemanticVersion>) -> String {
    let title = match version {
        Some(v) => format!("Changes from version {}", v.to_string()),
        None => "Latest changes".to_string(),
    };
    format.title()(&title)
}

const NO_SCOPE_TITLE: &str = "General";
const NON_CONVENTIONAL_TYPE: &str = "NON CONVENTIONAL";

const HASH_RANDOM_STATE: RandomState = RandomState::with_seeds(0, 0, 0, 0);
type ScopeMap = AHashMap<String, Vec<ConventionalCommit>>;
type TypeMap = AHashMap<String, ScopeMap>;

fn categorize_commit_list(
    list: impl Iterator<Item = Commit>,
    exclude_trigger: &Option<Trigger>,
) -> TypeMap {
    let mut types_map: TypeMap = AHashMap::with_hasher(HASH_RANDOM_STATE);
    list.for_each(|c| {
        let surely_conventional = match c {
            Commit::Conventional(commit) => commit,
            Commit::FreeForm(free_form) => {
                ConventionalCommit::new(NON_CONVENTIONAL_TYPE.to_owned(), None, false, free_form)
            }
        };
        if !exclude_trigger.as_ref().is_some_and(|it| {
            it.accept(
                surely_conventional.typ(),
                surely_conventional.scope(),
                surely_conventional.breaking(),
            )
        }) {
            ensure_inner_map_exists(&mut types_map, surely_conventional.typ());
            let scopes_map = types_map
                .get_mut(surely_conventional.typ())
                .expect("The map is ensured to exist");
            let scope = scope_or_general(&surely_conventional.scope());
            ensure_inner_vector_exists(scopes_map, &scope);
            scopes_map
                .get_mut(&scope)
                .expect("The vector is ensured to exist")
                .push(surely_conventional);
        }
    });
    types_map
}

#[inline(always)]
fn ensure_inner_map_exists(types_map: &mut TypeMap, t: &str) {
    if !types_map.contains_key(t) {
        types_map.insert(t.to_owned(), AHashMap::with_hasher(HASH_RANDOM_STATE));
    }
}

#[inline(always)]
fn ensure_inner_vector_exists(scope_map: &mut ScopeMap, s: &String) {
    if !scope_map.contains_key(s) {
        scope_map.insert(s.to_owned(), Vec::new());
    }
}

fn format_types(format: &ChangelogFormat, types_map: &TypeMap) -> String {
    let feat_scopes = types_map.get("feat").map_or(String::new(), |scope_map| {
        format!(
            "{}\n{}\n\n",
            format.typ()(&"feat".to_owned()),
            format_scopes(format, scope_map)
        )
    });

    let fix_scopes = types_map.get("fix").map_or(String::new(), |scope_map| {
        format!(
            "{}\n{}\n\n",
            format.typ()(&"fix".to_owned()),
            format_scopes(format, scope_map)
        )
    });

    let non_conventional =
        types_map
            .get(NON_CONVENTIONAL_TYPE)
            .map_or(String::new(), |scope_map| {
                format!(
                    "{}\n{}\n\n",
                    format.typ()(&NON_CONVENTIONAL_TYPE.to_owned()),
                    format_scopes(format, scope_map)
                )
            });

    feat_scopes
        + &fix_scopes
        + &types_map
            .iter()
            .filter(|(key, _)| *key != "feat" && *key != "fix")
            .map(|(key, value)| {
                format!("{}\n{}\n", format.typ()(key), format_scopes(format, value))
            })
            .reduce(|acc, e| acc + "\n" + &e)
            .unwrap_or_else(String::new)
        + &non_conventional
}

fn format_scopes(format: &ChangelogFormat, scope_map: &ScopeMap) -> String {
    scope_map
        .iter()
        .map(|(key, value)| {
            format!(
                "{}\n{}",
                format.scope()(key),
                format_list(format, value.iter())
            )
        })
        .reduce(|acc, e| acc + "\n" + &e)
        .unwrap_or_else(String::new)
}

fn format_list<'a>(
    format: &ChangelogFormat,
    commit_list: impl Iterator<Item = &'a ConventionalCommit>,
) -> String {
    format.list()(
        &commit_list
            .map(|c| format_item(format, c))
            .reduce(|acc, e| acc + "\n" + &e)
            .unwrap_or_else(String::new),
    )
}

fn format_item(format: &ChangelogFormat, commit: &ConventionalCommit) -> String {
    format.item()(&format_details(format, commit))
}

fn format_details(format: &ChangelogFormat, commit: &ConventionalCommit) -> String {
    if commit.breaking() {
        format.breaking()(commit.summary())
    } else {
        commit.summary().to_string()
    }
}

#[inline(always)]
fn scope_or_general(s: &Option<String>) -> String {
    match s {
        Some(expr) => expr.to_string(),
        None => String::from(NO_SCOPE_TITLE),
    }
}

#[cfg(test)]
mod tests {
    use ahash::AHashMap;

    use crate::{
        domain::{
            commit::Commit,
            configuration::changelog::{ChangelogConfiguration, ChangelogFormat},
            conventional_commit::ConventionalCommit,
            semantic_version::SemanticVersion,
            trigger::{BasicStatement, Trigger},
        },
        usecases::{
            repository::{
                commit_repository::CommitRepository, version_repository::VersionRepository,
            },
            usecases::{
                create_changelog::{
                    categorize_commit_list, ensure_inner_map_exists, ensure_inner_vector_exists,
                    format_details, format_item, format_list, format_scopes, format_title,
                    format_types, scope_or_general, CreateChangelogUseCase, ScopeMap, TypeMap,
                    HASH_RANDOM_STATE, NO_SCOPE_TITLE,
                },
                usecase::UseCase,
            },
        },
    };

    fn complete_commit() -> ConventionalCommit {
        ConventionalCommit::new(
            "feat".to_string(),
            Some("API".to_string()),
            false,
            "test message #1".to_string(),
        )
    }

    fn breaking_commit() -> ConventionalCommit {
        ConventionalCommit::new(
            "feat".to_string(),
            Some("API".to_string()),
            true,
            "test message #1".to_string(),
        )
    }

    fn commit_list() -> Vec<ConventionalCommit> {
        vec![
            ConventionalCommit::new(
                "feat".to_string(),
                Some("API".to_string()),
                false,
                "test message #1".to_string(),
            ),
            ConventionalCommit::new(
                "fix".to_string(),
                Some("API".to_string()),
                false,
                "test message #2".to_string(),
            ),
            ConventionalCommit::new(
                "test".to_string(),
                None,
                false,
                "test message #3".to_string(),
            ),
            ConventionalCommit::new(
                "refactor".to_string(),
                Some("exclude".to_string()),
                false,
                "test message #4".to_string(),
            ),
            ConventionalCommit::new(
                "docs".to_string(),
                None,
                false,
                "test message #5".to_string(),
            ),
            ConventionalCommit::new(
                "feat".to_string(),
                None,
                false,
                "test message #6".to_string(),
            ),
            ConventionalCommit::new(
                "test".to_string(),
                Some("API".to_string()),
                false,
                "test message #7".to_string(),
            ),
        ]
    }

    fn format() -> ChangelogFormat {
        ChangelogFormat::new(
            Box::new(|t| format!("# {}", t)),
            Box::new(|t| format!("## {}", t)),
            Box::new(|s| format!("### {}", s)),
            Box::new(|l| format!(":\n{}", l)),
            Box::new(|i| format!("* {}", i)),
            Box::new(|b| format!("**{}**", b)),
        )
    }

    #[test]
    fn format_details_not_breaking() {
        let c = complete_commit();
        let s = format_details(&format(), &c);
        assert_eq!(s, "test message #1".to_string());
    }

    #[test]
    fn format_details_breaking() {
        let c = breaking_commit();
        let s = format_details(&format(), &c);
        assert_eq!(s, "**test message #1**".to_string());
    }

    #[test]
    fn format_item_basic() {
        let c = complete_commit();
        let s = format_item(&format(), &c);
        assert_eq!(s, "* test message #1".to_string());
    }

    #[test]
    fn format_list_basic() {
        let l = commit_list();
        let s = format_list(&format(), l.iter());
        assert_eq!(
            s,
            format!(
                ":\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                "* test message #1",
                "* test message #2",
                "* test message #3",
                "* test message #4",
                "* test message #5",
                "* test message #6",
                "* test message #7",
            )
        )
    }

    #[test]
    fn format_list_empty() {
        let s = format_list(&format(), vec![].iter());
        assert_eq!(s, ":\n".to_string());
    }

    #[test]
    fn scope_or_general_some() {
        let s = scope_or_general(&Some("test".to_string()));
        assert_eq!(s, "test".to_string());
    }

    #[test]
    fn scope_or_general_empty() {
        let s = scope_or_general(&None);
        assert_eq!(s, NO_SCOPE_TITLE.to_owned());
    }

    #[test]
    fn format_scopes_basic() {
        let mut scope_map: ScopeMap = AHashMap::with_hasher(HASH_RANDOM_STATE);
        let l = commit_list();
        l.iter().for_each(|c| {
            let scope = scope_or_general(c.scope());
            ensure_inner_vector_exists(&mut scope_map, &scope);
            scope_map.get_mut(&scope).unwrap().push(c.clone());
        });
        let s = format_scopes(&format(), &scope_map);
        assert_eq!(
            s,
            format!("### API\n:\n* test message #1\n* test message #2\n* test message #7\n### exclude\n:\n* test message #4\n### General\n:\n* test message #3\n* test message #5\n* test message #6")
        );
    }

    #[test]
    fn categorize_commit_list_basic() {
        let m = categorize_commit_list(
            commit_list()
                .iter()
                .map(|it| Commit::Conventional(it.clone())),
            &None,
        );
        let expected = {
            let mut temp: TypeMap = AHashMap::with_hasher(HASH_RANDOM_STATE);

            ensure_inner_map_exists(&mut temp, "feat");
            let feat_commits = temp.get_mut("feat").expect("Map just created");
            ensure_inner_vector_exists(feat_commits, &"API".to_string());
            feat_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "feat".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #1".to_string(),
                ));
            ensure_inner_vector_exists(feat_commits, &NO_SCOPE_TITLE.to_string());
            feat_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "feat".to_string(),
                    None,
                    false,
                    "test message #6".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "fix");
            let fix_commits = temp.get_mut("fix").expect("Map just created");
            ensure_inner_vector_exists(fix_commits, &"API".to_string());
            fix_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "fix".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #2".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "test");
            let test_commits = temp.get_mut("test").expect("Map just created");
            ensure_inner_vector_exists(test_commits, &"API".to_string());
            test_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "test".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #7".to_string(),
                ));
            ensure_inner_vector_exists(test_commits, &NO_SCOPE_TITLE.to_string());
            test_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "test".to_string(),
                    None,
                    false,
                    "test message #3".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "refactor");
            let refactor_commits = temp.get_mut("refactor").expect("Map just created");
            ensure_inner_vector_exists(refactor_commits, &"exclude".to_string());
            refactor_commits
                .get_mut("exclude")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "refactor".to_string(),
                    Some("exclude".to_string()),
                    false,
                    "test message #4".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "docs");
            let docs_commits = temp.get_mut("docs").expect("Map just created");
            ensure_inner_vector_exists(docs_commits, &NO_SCOPE_TITLE.to_string());
            docs_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "docs".to_string(),
                    None,
                    false,
                    "test message #5".to_string(),
                ));
            temp
        };
        assert_eq!(m, expected);
    }

    #[test]
    fn categorize_commit_list_with_exclude_trigger() {
        let m = categorize_commit_list(
            commit_list()
                .iter()
                .map(|it| Commit::Conventional(it.clone())),
            &Some(Trigger::new(crate::domain::trigger::Start::Basic(
                BasicStatement::In(crate::domain::trigger::InNode {
                    object: crate::domain::trigger::ObjectNode::Scope(
                        crate::domain::trigger::ScopeNode {},
                    ),
                    array: crate::domain::trigger::ArrayNode {
                        values: vec!["exclude".to_string()],
                    },
                }),
            ))),
        );
        let expected = {
            let mut temp: TypeMap = AHashMap::with_hasher(HASH_RANDOM_STATE);

            ensure_inner_map_exists(&mut temp, "feat");
            let feat_commits = temp.get_mut("feat").expect("Map just created");
            ensure_inner_vector_exists(feat_commits, &"API".to_string());
            feat_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "feat".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #1".to_string(),
                ));
            ensure_inner_vector_exists(feat_commits, &NO_SCOPE_TITLE.to_string());
            feat_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "feat".to_string(),
                    None,
                    false,
                    "test message #6".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "fix");
            let fix_commits = temp.get_mut("fix").expect("Map just created");
            ensure_inner_vector_exists(fix_commits, &"API".to_string());
            fix_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "fix".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #2".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "test");
            let test_commits = temp.get_mut("test").expect("Map just created");
            ensure_inner_vector_exists(test_commits, &"API".to_string());
            test_commits
                .get_mut("API")
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "test".to_string(),
                    Some("API".to_string()),
                    false,
                    "test message #7".to_string(),
                ));
            ensure_inner_vector_exists(test_commits, &NO_SCOPE_TITLE.to_string());
            test_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "test".to_string(),
                    None,
                    false,
                    "test message #3".to_string(),
                ));

            ensure_inner_map_exists(&mut temp, "docs");
            let docs_commits = temp.get_mut("docs").expect("Map just created");
            ensure_inner_vector_exists(docs_commits, &NO_SCOPE_TITLE.to_string());
            docs_commits
                .get_mut(NO_SCOPE_TITLE)
                .expect("Vector just created")
                .push(ConventionalCommit::new(
                    "docs".to_string(),
                    None,
                    false,
                    "test message #5".to_string(),
                ));
            temp
        };
        assert_eq!(m, expected);
    }

    #[test]
    fn format_types_basic() {
        let c = commit_list();
        let s = format_types(
            &format(),
            &categorize_commit_list(c.iter().map(|it| Commit::Conventional(it.clone())), &None),
        );
        assert_eq!(s, "## feat\n### API\n:\n* test message #1\n### General\n:\n* test message #6\n\n## fix\n### API\n:\n* test message #2\n\n## docs\n### General\n:\n* test message #5\n\n## test\n### API\n:\n* test message #7\n### General\n:\n* test message #3\n\n## refactor\n### exclude\n:\n* test message #4\n");
    }

    #[test]
    fn format_title_basic() {
        let v = Some(SemanticVersion::first_release());
        let s = format_title(&format(), &v);
        assert_eq!(s, "# Changes from version 0.1.0");
    }

    #[test]
    fn format_title_empty_version() {
        let v = None;
        let s = format_title(&format(), &v);
        assert_eq!(s, "# Latest changes");
    }

    struct MockCommitRepository {}

    impl CommitRepository for MockCommitRepository {
        fn get_all_commits(&self) -> Box<dyn DoubleEndedIterator<Item = Commit>> {
            Box::new(
                commit_list()
                    .into_iter()
                    .map(|c| Commit::Conventional(c.clone())),
            )
        }

        fn get_commits_from(
            &self,
            _version: &Option<SemanticVersion>,
        ) -> Box<dyn DoubleEndedIterator<Item = Commit>> {
            Box::new(
                commit_list()
                    .into_iter()
                    .map(|c| Commit::Conventional(c.clone())),
            )
        }
    }

    struct MockVersionRepository {}

    impl VersionRepository for MockVersionRepository {
        fn last_version(&self) -> Option<SemanticVersion> {
            Some(SemanticVersion::new(
                0,
                1,
                0,
                Some("dev1".to_string()),
                None,
            ))
        }

        fn last_stable_version(&self) -> Option<SemanticVersion> {
            Some(SemanticVersion::first_release())
        }
    }

    #[test]
    fn execute_basic() {
        let configuration = ChangelogConfiguration::new(false, format(), None);
        let commit_repository = MockCommitRepository {};
        let version_repository = MockVersionRepository {};
        let usecase = CreateChangelogUseCase::new(
            configuration,
            Box::new(commit_repository),
            Box::new(version_repository),
        );
        let changelog = usecase.execute();
        assert_eq!(changelog, "# Changes from version 0.1.0\n## feat\n### API\n:\n* test message #1\n### General\n:\n* test message #6\n\n## fix\n### API\n:\n* test message #2\n\n## docs\n### General\n:\n* test message #5\n\n## test\n### API\n:\n* test message #7\n### General\n:\n* test message #3\n\n## refactor\n### exclude\n:\n* test message #4\n");
    }

    #[test]
    fn execute_from_latest_version() {
        let configuration = ChangelogConfiguration::new(true, format(), None);
        let commit_repository = MockCommitRepository {};
        let version_repository = MockVersionRepository {};
        let usecase = CreateChangelogUseCase::new(
            configuration,
            Box::new(commit_repository),
            Box::new(version_repository),
        );
        let changelog = usecase.execute();
        assert_eq!(changelog, "# Changes from version 0.1.0-dev1\n## feat\n### API\n:\n* test message #1\n### General\n:\n* test message #6\n\n## fix\n### API\n:\n* test message #2\n\n## docs\n### General\n:\n* test message #5\n\n## test\n### API\n:\n* test message #7\n### General\n:\n* test message #3\n\n## refactor\n### exclude\n:\n* test message #4\n");
    }

    #[test]
    fn execute_with_trigger() {
        let trigger = Some(Trigger::new(crate::domain::trigger::Start::Basic(
            BasicStatement::In(crate::domain::trigger::InNode {
                object: crate::domain::trigger::ObjectNode::Scope(
                    crate::domain::trigger::ScopeNode {},
                ),
                array: crate::domain::trigger::ArrayNode {
                    values: vec!["exclude".to_string()],
                },
            }),
        )));
        let configuration = ChangelogConfiguration::new(false, format(), trigger);
        let commit_repository = MockCommitRepository {};
        let version_repository = MockVersionRepository {};
        let usecase = CreateChangelogUseCase::new(
            configuration,
            Box::new(commit_repository),
            Box::new(version_repository),
        );
        let changelog = usecase.execute();
        assert_eq!(changelog, "# Changes from version 0.1.0\n## feat\n### API\n:\n* test message #1\n### General\n:\n* test message #6\n\n## fix\n### API\n:\n* test message #2\n\n## docs\n### General\n:\n* test message #5\n\n## test\n### API\n:\n* test message #7\n### General\n:\n* test message #3\n");
    }

    #[test]
    fn execute_from_latest_version_with_trigger() {
        let trigger = Some(Trigger::new(crate::domain::trigger::Start::Basic(
            BasicStatement::In(crate::domain::trigger::InNode {
                object: crate::domain::trigger::ObjectNode::Scope(
                    crate::domain::trigger::ScopeNode {},
                ),
                array: crate::domain::trigger::ArrayNode {
                    values: vec!["exclude".to_string()],
                },
            }),
        )));
        let configuration = ChangelogConfiguration::new(true, format(), trigger);
        let commit_repository = MockCommitRepository {};
        let version_repository = MockVersionRepository {};
        let usecase = CreateChangelogUseCase::new(
            configuration,
            Box::new(commit_repository),
            Box::new(version_repository),
        );
        let changelog = usecase.execute();
        assert_eq!(changelog, "# Changes from version 0.1.0-dev1\n## feat\n### API\n:\n* test message #1\n### General\n:\n* test message #6\n\n## fix\n### API\n:\n* test message #2\n\n## docs\n### General\n:\n* test message #5\n\n## test\n### API\n:\n* test message #7\n### General\n:\n* test message #3\n");
    }
}