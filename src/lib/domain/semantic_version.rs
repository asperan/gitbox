use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd)]
pub struct SemanticVersion {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Option<String>,
    metadata: Option<String>,
}

impl SemanticVersion {
    pub fn new(
        major: u32,
        minor: u32,
        patch: u32,
        prerelease: Option<String>,
        metadata: Option<String>,
    ) -> SemanticVersion {
        SemanticVersion {
            major,
            minor,
            patch,
            prerelease,
            metadata,
        }
    }

    pub fn first_release() -> SemanticVersion {
        SemanticVersion::new(0, 1, 0, None, None)
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }

    pub fn patch(&self) -> u32 {
        self.patch
    }

    pub fn prerelease(&self) -> &Option<String> {
        &self.prerelease
    }

    pub fn metadata(&self) -> &Option<String> {
        &self.metadata
    }
}

impl Ord for SemanticVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        let major_cmp_result = self.major.cmp(&other.major);
        if major_cmp_result == Ordering::Equal {
            let minor_cmp_result = self.minor.cmp(&other.minor);
            if minor_cmp_result == Ordering::Equal {
                let patch_cmp_result = self.patch.cmp(&other.patch());
                if patch_cmp_result == Ordering::Equal {
                    match (&self.prerelease, &other.prerelease) {
                        (Some(p1), Some(p2)) => p1.cmp(p2),
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        (None, None) => Ordering::Equal,
                    }
                } else {
                    patch_cmp_result
                }
            } else {
                minor_cmp_result
            }
        } else {
            major_cmp_result
        }
    }
}

impl Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prerelease_str = self
            .prerelease
            .as_ref()
            .map_or(String::new(), |p| format!("-{}", &p));
        let metadata_str = self
            .metadata
            .as_ref()
            .map_or(String::new(), |m| format!("+{}", &m));
        write!(
            f,
            "{}.{}.{}{}{}",
            self.major, self.minor, self.patch, prerelease_str, metadata_str
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::SemanticVersion;

    #[test]
    fn equal_versions_ordering() {
        let v1 = SemanticVersion::first_release();
        let v2 = SemanticVersion::first_release();
        assert_eq!(&v1.partial_cmp(&v2) == &Some(Ordering::Equal), v1 == v2);
    }

    #[test]
    fn less_versions_ordering() {
        let v1 = SemanticVersion::first_release();
        let v2 = SemanticVersion::new(1, 0, 0, None, None);
        assert_eq!(v1.partial_cmp(&v2) == Some(Ordering::Less), v1 < v2);
    }

    #[test]
    fn greater_versions_ordering() {
        let v1 = SemanticVersion::new(0, 1, 1, None, None);
        let v2 = SemanticVersion::first_release();
        assert_eq!(v1.partial_cmp(&v2) == Some(Ordering::Greater), v1 > v2);
    }

    #[test]
    fn less_or_equal_versions_ordering() {
        let v1 = SemanticVersion::first_release();
        let v2 = SemanticVersion::new(1, 0, 0, None, None);
        let v3 = SemanticVersion::first_release();

        let partial_cmp_result = v1.partial_cmp(&v2);
        assert_eq!(partial_cmp_result == Some(Ordering::Less) || partial_cmp_result == Some(Ordering::Equal), v1 <= v2);

        let partial_cmp_result = v1.partial_cmp(&v3);
        assert_eq!(partial_cmp_result == Some(Ordering::Less) || partial_cmp_result == Some(Ordering::Equal), v1 <= v3);
    }

    #[test]
    fn greater_or_equal_versions_ordering() {
        let v1 = SemanticVersion::new(0, 1, 1, None, None);
        let v2 = SemanticVersion::first_release();
        let v3 = SemanticVersion::new(0, 1, 1, None, None);
        let partial_cmp_result = v1.partial_cmp(&v2);
        assert_eq!(partial_cmp_result == Some(Ordering::Greater) || partial_cmp_result == Some(Ordering::Equal), v1 >= v2);

        let partial_cmp_result = v1.partial_cmp(&v3);
        assert_eq!(partial_cmp_result == Some(Ordering::Greater) || partial_cmp_result == Some(Ordering::Equal), v1 >= v3);
    }

    #[test]
    fn format () {
        let v1 = SemanticVersion::first_release();
        assert_eq!(v1.to_string(), String::from("0.1.0"));
    }
}
