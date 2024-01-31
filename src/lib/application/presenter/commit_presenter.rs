use std::{fmt::Display, str::FromStr};

use regex::Regex;

use crate::domain::{commit::Commit, conventional_commit::ConventionalCommit};

// Groups: 1 = type, 2 = scope with (), 3 = scope, 4 = breaking change, 5 = summary
const CONVENTIONAL_COMMIT_PATTERN: &str = r"^(\w+)(\(([\w/-]+)\))?(!)?: (.+)$";

impl FromStr for Commit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(CONVENTIONAL_COMMIT_PATTERN)
            .expect("The regex pattern is expected to be correct");
        let captures = regex.captures(s);
        match captures {
            Some(caps) => {
                let commit_type = caps.get(1).expect("Type should be expected").as_str();
                let scope = caps.get(3).map(|it| it.as_str());
                let breaking = caps.get(4).is_some();
                let summary = caps.get(5).expect("summary is expected").as_str();
                Ok(Commit::Conventional(ConventionalCommit::new(
                    commit_type.to_owned(),
                    scope.map(|it| it.to_owned()),
                    breaking,
                    summary.to_owned(),
                )))
            }
            None => Ok(Commit::FreeForm(s.to_owned())),
        }
    }
}

impl Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}: {}",
            &self.typ(),
            &self
                .scope()
                .as_ref()
                .map_or(String::new(), |s| format!("({})", s)),
            if self.breaking() { "!" } else { "" },
            &self.summary()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::domain::{commit::Commit, conventional_commit::ConventionalCommit};

    #[test]
    fn freeform_commit() {
        let free_form_commit = "Test update #1";
        let c = Commit::from_str(free_form_commit);
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            Commit::FreeForm(s) if s == free_form_commit.to_owned() => true,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_basic() {
        let basic_commit = "feat: test";
        let c = Commit::from_str(basic_commit);
        let expected = ConventionalCommit::new("feat".to_string(), None, false, "test".to_string());
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            Commit::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_scoped() {
        let scoped_commit = "feat(scope): test";
        let c = Commit::from_str(scoped_commit);
        let expected = ConventionalCommit::new(
            "feat".to_string(),
            Some("scope".to_string()),
            false,
            "test".to_string(),
        );
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            Commit::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_breaking() {
        let breaking_commit = "feat!: test";
        let c = Commit::from_str(breaking_commit);
        let expected = ConventionalCommit::new("feat".to_string(), None, true, "test".to_string());
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            Commit::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_scoped_and_breaking() {
        let breaking_scoped_commit = "feat(scope)!: test";
        let c = Commit::from_str(breaking_scoped_commit);
        let expected = ConventionalCommit::new(
            "feat".to_string(),
            Some("scope".to_string()),
            true,
            "test".to_string(),
        );
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            Commit::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn simple_commit_format() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test format".to_string());
        assert_eq!(&commit.to_string(), "feat: test format");
    }

    #[test]
    fn scoped_commit_format() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            false,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain): test format");
    }

    #[test]
    fn breaking_commit_format() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, true, "test format".to_string());
        assert_eq!(&commit.to_string(), "feat!: test format");
    }

    #[test]
    fn breaking_and_scoped_commit_format() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            true,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain)!: test format");
    }
}
