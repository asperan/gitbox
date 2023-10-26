use crate::common::{cached_values::CachedValues, git::{FIRST_STABLE_RELEASE, commit_list}};

use super::change_parser::{Trigger, ChangeTriggerParser};

type DefaultChangeAcceptor = Box<dyn Fn(&str, &Option<String>, bool) -> bool>;

enum TriggerWrapper {
    Default(DefaultChangeAcceptor),
    Custom(Trigger),
}

impl TriggerWrapper {
    fn accept(&self, commit_type: &str, scope: &Option<String>, breaking: bool) -> bool {
        match self {
            TriggerWrapper::Default(function) => function(commit_type, scope, breaking),
            TriggerWrapper::Custom(trigger) => trigger.call(commit_type, scope, breaking),
        }
    }
}

pub struct StableVersionCalculator {
    major_trigger: TriggerWrapper,
    minor_trigger: TriggerWrapper,
    patch_trigger: TriggerWrapper,
}

impl StableVersionCalculator {
    pub fn new(major_str: &Option<String>, minor_str: &Option<String>, patch_str: &Option<String>) -> StableVersionCalculator {
        StableVersionCalculator {
            major_trigger: match major_str {
                Some(s) => TriggerWrapper::Custom(ChangeTriggerParser::run(s)),
                None => TriggerWrapper::Default(Box::new(StableVersionCalculator::default_major_trigger)),
            },
            minor_trigger: match minor_str {
                Some(s) => TriggerWrapper::Custom(ChangeTriggerParser::run(s)),
                None => TriggerWrapper::Default(Box::new(StableVersionCalculator::default_minor_trigger)),
            },
            patch_trigger: match patch_str {
                Some(s) => TriggerWrapper::Custom(ChangeTriggerParser::run(s)),
                None => TriggerWrapper::Default(Box::new(StableVersionCalculator::default_patch_trigger)),
            }
        }
    }

    pub fn next_stable(&self, last_stable: &Option<String>) -> String {
        match last_stable {
            Some(version) => {
                let commit_list = commit_list(Some(version));
                let captures = CachedValues::semantic_version_regex().captures(version).unwrap();
                let major: u16 = captures.get(2).unwrap().as_str().parse().unwrap();
                let minor: u16 = captures.get(3).unwrap().as_str().parse().unwrap();
                let patch: u16 = captures.get(4).unwrap().as_str().parse().unwrap();
                let max_change = commit_list.iter().filter_map(|c| self.message_to_change(c)).max();
                match max_change {
                    Some(Change::Major) => format!("{}.{}.{}", major + 1, 0, 0),
                    Some(Change::Minor) => format!("{}.{}.{}", major, minor + 1, 0),
                    Some(Change::Patch) => format!("{}.{}.{}", major, minor, patch + 1),
                    _ => version.to_owned(),
                }
            },
            None => FIRST_STABLE_RELEASE.to_string(),
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
        },
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


