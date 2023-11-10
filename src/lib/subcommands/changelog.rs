use ahash::{AHashMap, RandomState};
use clap::Args;

use crate::common::{
    cached_values::CachedValues,
    commons::print_error_and_exit,
    semantic_version::SemanticVersion,
    trigger::Trigger,
};

const NO_SCOPE_TITLE: &str = "General";
const NON_CONVENTIONAL_TYPE: &str = "Non conventional";
const HASH_RANDOM_STATE: RandomState = RandomState::with_seeds(0, 0, 0, 0);

// CommitDetails contains the message and the breakingness of the commit
type CommitDetails = (String, bool);
type ScopeMap = AHashMap<String, Vec<CommitDetails>>;
type TypeMap = AHashMap<String, ScopeMap>;

#[derive(Args, Debug)]
#[command(about = "Generate a changelog")]
pub struct ChangelogSubCommand {
    #[arg(
        long,
        help = "If set, the changelog will be generated with changes since the last version rather than the last stable release"
    )]
    from_latest_version: bool,
    #[arg(
        short = 'T',
        long,
        help = "Set the title format. The content placeholder is '%s'",
        default_value("# %s"),
        allow_hyphen_values(true)
    )]
    title_format: String,
    #[arg(
        short = 't',
        long,
        help = "Set the type format. The content placeholder is '%s'",
        default_value("= %s"),
        allow_hyphen_values(true)
    )]
    type_format: String,
    #[arg(
        short = 's',
        long,
        help = "Set the scope format. The content placeholder is '%s'",
        default_value("- %s"),
        allow_hyphen_values(true)
    )]
    scope_format: String,
    #[arg(
        short = 'l',
        long,
        help = "Set the list format. The content placeholder is '%s'",
        default_value("%s"),
        allow_hyphen_values(true)
    )]
    list_format: String,
    #[arg(
        short = 'i',
        long,
        help = "Set the list item format. The content placeholder is '%s'",
        default_value("* %s"),
        allow_hyphen_values(true)
    )]
    item_format: String,
    #[arg(
        short = 'b',
        long,
        help = "Set the breaking commit format. The content placeholder is '%s'",
        default_value("!!! %s "),
        allow_hyphen_values(true)
    )]
    breaking_format: String,

    #[arg(
        long,
        help = "Set the trigger to use to exclude commits from the changelog. For more informations about the grammar, run 'help grammar'"
    )]
    exclude_trigger: Option<String>,
}

impl ChangelogSubCommand {
    pub fn changelog(&self) {
        self.ensure_formats_have_placeholder();
        let types_map = self.categorize_commits_from(if self.from_latest_version {
            CachedValues::last_version()
        } else {
            CachedValues::last_stable_release()
        });
        if types_map.len() == 0usize {
            println!("No changes since last {}", if self.from_latest_version { "version" } else { "stable release" });
        } else {
            println!("{}", self.format_types(&types_map).trim());
        }
    }

    fn categorize_commits_from(&self, version: &'static Option<SemanticVersion>) -> TypeMap {
        let exclude_trigger = self.exclude_trigger.as_ref().map(|s| Trigger::from(s));
        let commit_list = CachedValues::single_branch_commit_list(version.as_ref());
        let mut types_map: TypeMap = AHashMap::with_hasher(HASH_RANDOM_STATE);
        commit_list.iter().for_each(|c| {
            let captures = CachedValues::conventional_commit_regex().captures(c);
            match captures {
                Some(caps) => {
                    let commit_type = caps.get(1).unwrap().as_str();
                    let scope = caps.get(3).map_or(NO_SCOPE_TITLE, |m| m.as_str());
                    let is_breaking = caps.get(4).is_some();
                    let message = caps.get(5).unwrap().as_str();
                    if !exclude_trigger.as_ref().is_some_and(|t| {
                        t.accept(commit_type, &Some(scope.to_owned()), is_breaking)
                    }) {
                        Self::ensure_inner_map_exists(&mut types_map, &commit_type.to_owned());
                        let scopes_map = types_map.get_mut(commit_type).unwrap();
                        Self::ensure_inner_vector_exists(scopes_map, &scope.to_owned());
                        scopes_map
                            .get_mut(scope)
                            .unwrap()
                            .push((message.trim().to_string(), is_breaking));
                    }
                }
                None => {
                    Self::ensure_inner_map_exists(
                        &mut types_map,
                        &NON_CONVENTIONAL_TYPE.to_owned(),
                    );
                    let non_conventional_map = types_map.get_mut(NON_CONVENTIONAL_TYPE).unwrap();
                    Self::ensure_inner_vector_exists(
                        non_conventional_map,
                        &NO_SCOPE_TITLE.to_owned(),
                    );
                    non_conventional_map
                        .get_mut(NO_SCOPE_TITLE)
                        .unwrap()
                        .push((c.to_owned(), false));
                }
            }
        });
        types_map
    }

    fn format_types(&self, types_map: &TypeMap) -> String {
        let feat_scopes = types_map.get("feat").map_or(String::from(""), |scope_map| {
            format!(
                "{}\n{}\n\n",
                self.type_format.replace("%s", "feat"),
                self.format_scopes(scope_map)
            )
        });
        let fix_scopes = types_map.get("fix").map_or(String::from(""), |scope_map| {
            format!(
                "{}\n{}\n\n",
                self.type_format.replace("%s", "fix"),
                self.format_scopes(scope_map)
            )
        });
        feat_scopes
            + &fix_scopes
            + &types_map
                .iter()
                .filter(|(key, _)| *key != "feat" && *key != "fix")
                .map(|(key, value)| {
                    format!(
                        "{}\n{}\n",
                        self.type_format.replace("%s", key),
                        self.format_scopes(value)
                    )
                })
                .reduce(|acc, e| acc + "\n" + &e)
                .unwrap_or_else(|| String::from(""))
    }

    fn format_scopes(&self, scope_map: &ScopeMap) -> String {
        scope_map
            .iter()
            .map(|(key, value)| {
                format!(
                    "{}\n{}",
                    self.scope_format.replace("%s", key),
                    self.format_list(value)
                )
            })
            .reduce(|acc, e| acc + "\n" + &e)
            .unwrap_or_else(|| String::from(""))
    }

    fn format_list(&self, commit_list: &[CommitDetails]) -> String {
        commit_list
            .iter()
            .map(|cd| self.item_format.replace("%s", &self.format_details(cd)))
            .reduce(|acc, e| acc + "\n" + &e)
            .unwrap_or_else(|| String::from(""))
    }

    fn format_details(&self, details: &CommitDetails) -> String {
        if details.1 {
            self.breaking_format.replace("%s", &details.0)
        } else {
            details.0.clone()
        }
    }

    fn ensure_formats_have_placeholder(&self) {
        if !self.title_format.contains("%s") {
            print_error_and_exit("The given title format does not contain the placeholder '%s'");
        }
        if !self.type_format.contains("%s") {
            print_error_and_exit("The given type format does not contain the placeholder '%s'");
        }
        if !self.scope_format.contains("%s") {
            print_error_and_exit("The given scope format does not contain the placeholder '%s'");
        }
        if !self.list_format.contains("%s") {
            print_error_and_exit("The given list format does not contain the placeholder '%s'");
        }
        if !self.item_format.contains("%s") {
            print_error_and_exit("The given item format does not contain the placeholder '%s'");
        }
        if !self.breaking_format.contains("%s") {
            print_error_and_exit("The given breaking format does not contain the placeholder '%s'");
        }
    }

    fn ensure_inner_map_exists(commit_map: &mut TypeMap, key: &String) {
        if !commit_map.contains_key(key) {
            commit_map.insert(key.to_owned(), AHashMap::with_hasher(HASH_RANDOM_STATE));
        }
    }

    fn ensure_inner_vector_exists(scope_map: &mut ScopeMap, key: &String) {
        if !scope_map.contains_key(key) {
            scope_map.insert(key.to_owned(), Vec::new());
        }
    }
}
