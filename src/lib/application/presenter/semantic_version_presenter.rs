use std::{error::Error, fmt::Display, str::FromStr};
use regex::Regex;

use crate::domain::semantic_version::SemanticVersion;

const FULL_SEMANTIC_VERSION_PATTERN: &str = concat!(
    // GROUPS:
    // 1 = Stable version, 2 = major, 3 = minor, 4 = patch
    r"^((0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*))",
    // 5 = prerelease
    r"(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?",
    // 6 = metadata
    r"(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"
);

impl FromStr for SemanticVersion {
    type Err = SemanticVersionParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(FULL_SEMANTIC_VERSION_PATTERN).expect("The constant semantic version pattern should be correct");
        let captures = regex.captures(s);
        match captures {
            Some(caps) => {
                let major = caps.get(2).unwrap().as_str().parse().unwrap();
                let minor = caps.get(3).unwrap().as_str().parse().unwrap();
                let patch = caps.get(4).unwrap().as_str().parse().unwrap();
                let prerelease = caps.get(5).map(|m| m.as_str().to_owned());
                let metadata = caps.get(6).map(|m| m.as_str().to_owned());
                Ok(SemanticVersion::new(major, minor, patch, prerelease, metadata))
            }
            None => Err(SemanticVersionParsingError::new(s)),
        }
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prerelease_str = self
            .prerelease()
            .as_ref()
            .map_or(String::new(), |p| format!("-{}", &p));
        let metadata_str = self
            .metadata()
            .as_ref()
            .map_or(String::new(), |m| format!("+{}", &m));
        write!(
            f,
            "{}.{}.{}{}{}",
            self.major(), self.minor(), self.patch(), prerelease_str, metadata_str
        )
    }
}

#[derive(Debug)]
pub struct SemanticVersionParsingError {
    wrong_version: String,
}

impl SemanticVersionParsingError {
    fn new(wrong_version: &str) -> SemanticVersionParsingError {
        SemanticVersionParsingError { wrong_version: wrong_version.to_string() }
    }
}

impl Display for SemanticVersionParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version '{}' is not semantic", self.wrong_version)
    }
}

impl Error for SemanticVersionParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::domain::semantic_version::SemanticVersion;

    #[test]
    fn parse_simple_semantic_version() {
        let s = "1.3.15";
        let v = SemanticVersion::from_str(s);
        match v {
            Ok(version) => {
                assert_eq!(version.major(), 1);
                assert_eq!(version.minor(), 3);
                assert_eq!(version.patch(), 15);
                assert_eq!(version.prerelease(), &None);
                assert_eq!(version.metadata(), &None);
            },
            Err(_) => assert!(false, "The version should be parsable correctly"),
        }
    }

    #[test]
    fn parse_semantic_prerelease() {
        let s = "1.3.15-alpha1";
        let v = SemanticVersion::from_str(s);
        match v {
            Ok(version) => {
                assert_eq!(version.major(), 1);
                assert_eq!(version.minor(), 3);
                assert_eq!(version.patch(), 15);
                assert_eq!(version.prerelease(), &Some("alpha1".to_string()));
                assert_eq!(version.metadata(), &None);
            },
            Err(_) => assert!(false, "The version should be parsable correctly"),
        }
    }

    #[test]
    fn parse_semantic_version_with_metadata() {
        let s = "1.3.15+test";
        let v = SemanticVersion::from_str(s);
        match v {
            Ok(version) => {
                assert_eq!(version.major(), 1);
                assert_eq!(version.minor(), 3);
                assert_eq!(version.patch(), 15);
                assert_eq!(version.prerelease(), &None);
                assert_eq!(version.metadata(), &Some("test".to_string()));
            },
            Err(_) => assert!(false, "The version should be parsable correctly"),
        }

    }

    #[test]
    fn parse_semantic_prerelease_with_metadata() {
        let s = "1.3.15-alpha1+test";
        let v = SemanticVersion::from_str(s);
        match v {
            Ok(version) => {
                assert_eq!(version.major(), 1);
                assert_eq!(version.minor(), 3);
                assert_eq!(version.patch(), 15);
                assert_eq!(version.prerelease(), &Some("alpha1".to_string()));
                assert_eq!(version.metadata(), &Some("test".to_string()));
            },
            Err(_) => assert!(false, "The version should be parsable correctly"),
        }

    }

    #[test]
    fn try_parse_non_semantic_version() {
        let s = "1970-01-01";
        let v = SemanticVersion::from_str(s);
        assert!(v.is_err() && v.unwrap_err().wrong_version == s.to_string());
    }

    #[test]
    fn simple_version_format() {
        let v1 = SemanticVersion::first_release();
        assert_eq!(v1.to_string(), String::from("0.1.0"));
    }

    #[test]
    fn prerelease_version_format() {
        let v1 = SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), None);
        assert_eq!(v1.to_string(), String::from("0.1.0-dev1"));
    }

    #[test]
    fn version_with_metadata_format() {
        let v1 = SemanticVersion::new(0, 1, 0, None, Some("test".to_string()));
        assert_eq!(v1.to_string(), String::from("0.1.0+test"));
    }

    #[test]
    fn prerelease_version_with_metadata_format() {
        let v1 = SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), Some("test".to_string()));
        assert_eq!(v1.to_string(), String::from("0.1.0-dev1+test"));
    }
}
