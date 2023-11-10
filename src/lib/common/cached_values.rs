use ahash::AHashMap;
use regex::Regex;

use super::{
    git::{
        commit_list, git_dir, last_stable_version, last_version, CommitBranch,
        CONVENTIONAL_COMMIT_PATTERN, FULL_SEMANTIC_VERSION_PATTERN,
    },
    semantic_version::SemanticVersion,
};

pub struct CachedValues {}

impl CachedValues {
    pub fn git_dir() -> &'static String {
        unsafe {
            match &GIT_DIR {
                Some(value) => value,
                None => {
                    GIT_DIR = Some(git_dir());
                    GIT_DIR.as_ref().unwrap()
                }
            }
        }
    }

    pub fn last_version() -> &'static Option<SemanticVersion> {
        unsafe {
            match &LAST_VERSION {
                Some(value) => value,
                None => {
                    LAST_VERSION = Some(last_version());
                    LAST_VERSION.as_ref().unwrap()
                }
            }
        }
    }

    pub fn last_stable_release() -> &'static Option<SemanticVersion> {
        unsafe {
            match &LAST_RELEASE {
                Some(value) => value,
                None => {
                    LAST_RELEASE = Some(last_stable_version());
                    LAST_RELEASE.as_ref().unwrap()
                }
            }
        }
    }

    pub fn semantic_version_regex() -> &'static Regex {
        unsafe {
            match &SEMANTIC_VERSION_REGEX {
                Some(value) => value,
                None => {
                    SEMANTIC_VERSION_REGEX =
                        Some(Regex::new(FULL_SEMANTIC_VERSION_PATTERN).unwrap());
                    SEMANTIC_VERSION_REGEX.as_ref().unwrap()
                }
            }
        }
    }

    pub fn conventional_commit_regex() -> &'static Regex {
        unsafe {
            match &CONVENTIONAL_COMMIT_REGEX {
                Some(value) => value,
                None => {
                    CONVENTIONAL_COMMIT_REGEX =
                        Some(Regex::new(CONVENTIONAL_COMMIT_PATTERN).unwrap());
                    CONVENTIONAL_COMMIT_REGEX.as_ref().unwrap()
                }
            }
        }
    }

    pub fn single_branch_commit_list(from: Option<&'static SemanticVersion>) -> &Vec<String> {
        unsafe {
            if SINGLE_BRANCH_COMMIT_LISTS.is_none() {
                SINGLE_BRANCH_COMMIT_LISTS = Some(AHashMap::new())
            }
            let map = SINGLE_BRANCH_COMMIT_LISTS.as_mut().expect(
                "This option should always be Some, as if it wasn't, it has been crea ed now",
            );
            if !map.contains_key(&from) {
                map.insert(from, commit_list(from, CommitBranch::Single));
            }
            map.get(&from).as_ref().expect("The map always contains the key requested, as if it wasn't in the map, it has justs been added")
        }
    }

    pub fn all_branches_commit_list(from: Option<&'static SemanticVersion>) -> &Vec<String> {
        unsafe {
            if ALL_BRANCHES_COMMIT_LISTS.is_none() {
                ALL_BRANCHES_COMMIT_LISTS = Some(AHashMap::new())
            }
            let map = ALL_BRANCHES_COMMIT_LISTS.as_mut().expect(
                "This option should always be Some, as if it wasn't, it has been crea ed now",
            );
            if !map.contains_key(&from) {
                map.insert(from, commit_list(from, CommitBranch::All));
            }
            map.get(&from).as_ref().expect("The map always contains the key requested, as if it wasn't in the map, it has justs been added")
        }
    }
}

static mut GIT_DIR: Option<String> = None;
static mut LAST_VERSION: Option<Option<SemanticVersion>> = None;
static mut LAST_RELEASE: Option<Option<SemanticVersion>> = None;
static mut CONVENTIONAL_COMMIT_REGEX: Option<Regex> = None;
static mut SEMANTIC_VERSION_REGEX: Option<Regex> = None;
static mut SINGLE_BRANCH_COMMIT_LISTS: Option<AHashMap<Option<&SemanticVersion>, Vec<String>>> =
    None;
static mut ALL_BRANCHES_COMMIT_LISTS: Option<AHashMap<Option<&SemanticVersion>, Vec<String>>> =
    None;
