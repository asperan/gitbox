use crate::common::{
    cached_values::CachedValues, git::commit_list, semantic_version::SemanticVersion,
    trigger::Trigger,
};

type DefaultChangeAcceptor = Box<dyn Fn(&str, &Option<String>, bool) -> bool>;

enum TriggerWrapper {
    Default(DefaultChangeAcceptor),
    Custom(Trigger),
}

impl TriggerWrapper {
    fn accept(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            TriggerWrapper::Default(function) => function(commit_type, scope, breaking),
            TriggerWrapper::Custom(trigger) => trigger.accept(commit_type, scope, breaking),
        }
    }
}

pub struct StableVersionCalculator {
    major_trigger: TriggerWrapper,
    minor_trigger: TriggerWrapper,
    patch_trigger: TriggerWrapper,
}

impl StableVersionCalculator {
    pub fn new(
        major_str: &Option<String>,
        minor_str: &Option<String>,
        patch_str: &Option<String>,
    ) -> StableVersionCalculator {
        StableVersionCalculator {
            major_trigger: match major_str {
                Some(s) => TriggerWrapper::Custom(Trigger::from(s)),
                None => TriggerWrapper::Default(Box::new(
                    StableVersionCalculator::default_major_trigger,
                )),
            },
            minor_trigger: match minor_str {
                Some(s) => TriggerWrapper::Custom(Trigger::from(s)),
                None => TriggerWrapper::Default(Box::new(
                    StableVersionCalculator::default_minor_trigger,
                )),
            },
            patch_trigger: match patch_str {
                Some(s) => TriggerWrapper::Custom(Trigger::from(s)),
                None => TriggerWrapper::Default(Box::new(
                    StableVersionCalculator::default_patch_trigger,
                )),
            },
        }
    }

    pub fn next_stable(&self, last_stable: &Option<SemanticVersion>) -> SemanticVersion {
        match last_stable {
            Some(version) => {
                let commit_list = commit_list(Some(version));
                let max_change = commit_list
                    .iter()
                    .filter_map(|c| self.message_to_change(c))
                    .max();
                match max_change {
                    Some(Change::Major) => {
                        SemanticVersion::new(version.major() + 1, 0, 0, None, None)
                    }
                    Some(Change::Minor) => {
                        SemanticVersion::new(version.major(), version.minor() + 1, 0, None, None)
                    }
                    Some(Change::Patch) => SemanticVersion::new(
                        version.major(),
                        version.minor(),
                        version.patch() + 1,
                        None,
                        None,
                    ),
                    _ => SemanticVersion::new(
                        version.major(),
                        version.minor(),
                        version.patch(),
                        None,
                        None,
                    ),
                }
            }
            None => SemanticVersion::first_release(),
        }
    }

    fn default_major_trigger(_commit_typee: &str, _scope: &Option<String>, breaking: bool) -> bool {
        breaking
    }

    fn default_minor_trigger(commit_type: &str, _scope: &Option<String>, _breaking: bool) -> bool {
        commit_type == "feat"
    }

    fn default_patch_trigger(commit_type: &str, _scope: &Option<String>, _breaking: bool) -> bool {
        commit_type == "fix"
    }

    fn message_to_change(&self, message: &str) -> Option<Change> {
        let conv_commits_regex = CachedValues::conventional_commit_regex();
        let captures = conv_commits_regex.captures(message);
        match captures {
            Some(caps) => {
                let commit_type = caps.get(1).unwrap().as_str();
                let scope = caps.get(3).map(|t| t.as_str().to_string());
                let breaking = caps.get(4).is_some();
                if self.major_trigger.accept(commit_type, &scope, breaking) {
                    Some(Change::Major)
                } else if self.minor_trigger.accept(commit_type, &scope, breaking) {
                    Some(Change::Minor)
                } else if self.patch_trigger.accept(commit_type, &scope, breaking) {
                    Some(Change::Patch)
                } else {
                    Some(Change::None)
                }
            }
            None => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Change {
    None,
    Patch,
    Minor,
    Major,
}
