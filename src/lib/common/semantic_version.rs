use std::{cmp::Ordering, fmt::Display};

use super::{cached_values::CachedValues, commons::print_error_and_exit};

#[derive(Debug, PartialEq, Eq)]
pub struct SemanticVersion {
    major: u16,
    minor: u16,
    patch: u16,
    prerelease: Option<String>,
    metadata: Option<String>,
}

impl SemanticVersion {
    pub fn new(major: u16, minor: u16, patch: u16, prerelease: Option<String>, metadata: Option<String>) -> SemanticVersion {
        SemanticVersion { major, minor, patch, prerelease, metadata }
    }

    pub fn from_str(s: &str) -> SemanticVersion {
        let captures = CachedValues::semantic_version_regex().captures(s);
        match captures {
            Some(caps) => {
                let major = caps.get(2).unwrap().as_str().parse().unwrap();
                let minor = caps.get(3).unwrap().as_str().parse().unwrap();
                let patch = caps.get(4).unwrap().as_str().parse().unwrap();
                let prerelease = caps.get(5).map(|m| m.as_str().to_owned());
                let metadata = caps.get(6).map(|m| m.as_str().to_owned());
                SemanticVersion { major, minor, patch, prerelease, metadata }
            },
            None => print_error_and_exit(&format!("Failed to parse semantic version from string '{}'", &s)),
        }
    }

    pub fn first_release() -> SemanticVersion {
        SemanticVersion { major: 0, minor: 1, patch: 0, prerelease: None, metadata: None }
    }

    pub fn major(&self) -> u16 {
        self.major
    }

    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn patch(&self) -> u16 {
        self.patch
    }
}

impl PartialOrd<SemanticVersion> for SemanticVersion {
    fn partial_cmp(&self, other: &SemanticVersion) -> Option<Ordering> {
        match self.major.partial_cmp(&other.major) {
            Some(Ordering::Equal) => {
                match self.minor.partial_cmp(&other.minor) {
                    Some(Ordering::Equal) => {
                        match self.patch.partial_cmp(&other.patch) {
                            Some(Ordering::Equal) => {
                                match (&self.prerelease, &other.prerelease) {
                                    (Some(p1), Some(p2)) => p1.partial_cmp(p2),
                                    (Some(_), None) => Some(Ordering::Less),
                                    (None, Some(_)) => Some(Ordering::Greater),
                                    (None, None) => Some(Ordering::Equal),
                                }
                            },
                            Some(o) => Some(o),
                            None => None,
                        }
                    },
                    Some(o) => Some(o),
                    None => None,
                }
            },
            Some(o) => Some(o),
            None => None,
        }
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prerelease_str = self.prerelease.as_ref().map_or(String::new(), |p| format!("-{}", &p));
        let metadata_str = self.metadata.as_ref().map_or(String::new(), |m| format!("+{}", &m));
        write!(f, "{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            prerelease_str,
            metadata_str
        )
    }
}
