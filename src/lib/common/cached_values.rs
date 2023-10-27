use regex::Regex;

use super::{git::{git_dir, last_version, last_stable_version, FULL_SEMANTIC_VERSION_PATTERN, CONVENTIONAL_COMMIT_PATTERN}, semantic_version::SemanticVersion};

pub struct CachedValues {}

impl CachedValues {
    pub fn git_dir() -> &'static String {
        unsafe {
            match &GIT_DIR {
                Some(value) => value,
                None => {
                    GIT_DIR = Some(git_dir());
                    GIT_DIR.as_ref().unwrap()
                },
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
                },
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
                },
            }
        }
    }

    pub fn semantic_version_regex() -> &'static Regex {
        unsafe {
            match &SEMANTIC_VERSION_REGEX {
                Some(value) => value,
                None => {
                    SEMANTIC_VERSION_REGEX = Some(Regex::new(FULL_SEMANTIC_VERSION_PATTERN).unwrap());
                    SEMANTIC_VERSION_REGEX.as_ref().unwrap()
                },
            }
        }
    }

    pub fn conventional_commit_regex() -> &'static Regex {
        unsafe {
            match &CONVENTIONAL_COMMIT_REGEX {
                Some(value) => value,
                None => {
                    CONVENTIONAL_COMMIT_REGEX = Some(Regex::new(CONVENTIONAL_COMMIT_PATTERN).unwrap());
                    CONVENTIONAL_COMMIT_REGEX.as_ref().unwrap()
                },
            }
        }
    }
}

static mut GIT_DIR: Option<String> = None;
static mut LAST_VERSION: Option<Option<SemanticVersion>> = None;
static mut LAST_RELEASE: Option<Option<SemanticVersion>> = None;
static mut CONVENTIONAL_COMMIT_REGEX: Option<Regex> = None;
static mut SEMANTIC_VERSION_REGEX: Option<Regex> = None;
