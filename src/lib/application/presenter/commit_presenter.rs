use std::{fmt::Display, str::FromStr};

use regex::Regex;

use crate::{
    application::error::commit_summary_parsing_error::CommitSummaryParsingError,
    domain::{
        commit_summary::CommitSummary, conventional_commit::ConventionalCommit,
        conventional_commit_summary::ConventionalCommitSummary,
    },
    usecase::type_aliases::AnyError,
};

// Groups: 1 = type, 2 = scope with (), 3 = scope, 4 = breaking change, 5 = summary
const CONVENTIONAL_COMMIT_PATTERN: &str = r"^(\w+)(\(([\w/-]+)\))?(!)?: (.+)$";

impl FromStr for CommitSummary {
    type Err = AnyError;

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
                Ok(CommitSummary::Conventional(ConventionalCommitSummary::new(
                    commit_type.to_owned(),
                    scope.map(|it| it.to_owned()),
                    breaking,
                    summary.to_owned(),
                )))
            }
            None => {
                if s.is_empty() {
                    Err(Box::new(CommitSummaryParsingError::new(
                        "Free form commit message cannot be empty",
                    )))
                } else {
                    Ok(CommitSummary::FreeForm(s.to_owned()))
                }
            }
        }
    }
}

impl Display for ConventionalCommitSummary {
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

impl Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.summary(),
            self.message()
                .as_ref()
                .map_or_else(String::new, |it| format!("\n\n{}", it))
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::domain::{
        commit_summary::CommitSummary, conventional_commit::ConventionalCommit,
        conventional_commit_summary::ConventionalCommitSummary,
    };

    #[test]
    fn freeform_commit_correct() {
        let free_form_commit = "Test update #1";
        let c = CommitSummary::from_str(free_form_commit);
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            CommitSummary::FreeForm(s) if s == *free_form_commit => true,
            _ => false,
        });
    }

    #[test]
    fn freeform_commit_empty() {
        let free_form_commit = "";
        let c = CommitSummary::from_str(free_form_commit);
        assert!(c.is_err());
    }

    #[test]
    fn conventional_commit_basic() {
        let basic_commit = "feat: test";
        let c = CommitSummary::from_str(basic_commit);
        let expected =
            ConventionalCommitSummary::new("feat".to_string(), None, false, "test".to_string());
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            CommitSummary::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_scoped() {
        let scoped_commit = "feat(scope): test";
        let c = CommitSummary::from_str(scoped_commit);
        let expected = ConventionalCommitSummary::new(
            "feat".to_string(),
            Some("scope".to_string()),
            false,
            "test".to_string(),
        );
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            CommitSummary::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_breaking() {
        let breaking_commit = "feat!: test";
        let c = CommitSummary::from_str(breaking_commit);
        let expected =
            ConventionalCommitSummary::new("feat".to_string(), None, true, "test".to_string());
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            CommitSummary::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn conventional_commit_scoped_and_breaking() {
        let breaking_scoped_commit = "feat(scope)!: test";
        let c = CommitSummary::from_str(breaking_scoped_commit);
        let expected = ConventionalCommitSummary::new(
            "feat".to_string(),
            Some("scope".to_string()),
            true,
            "test".to_string(),
        );
        assert!(c.is_ok());
        assert!(match c.expect("Just asserted its OK-ness") {
            CommitSummary::Conventional(conv) => conv == expected,
            _ => false,
        });
    }

    #[test]
    fn simple_commit_format() {
        let commit = ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat: test format");
    }

    #[test]
    fn scoped_commit_format() {
        let commit = ConventionalCommitSummary::new(
            "feat".to_string(),
            Some("domain".to_string()),
            false,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain): test format");
    }

    #[test]
    fn breaking_commit_format() {
        let commit = ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            true,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat!: test format");
    }

    #[test]
    fn breaking_and_scoped_commit_format() {
        let commit = ConventionalCommitSummary::new(
            "feat".to_string(),
            Some("domain".to_string()),
            true,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain)!: test format");
    }

    #[test]
    fn full_conventional_commit_without_message() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            true,
            "test format".to_string(),
            None,
        );
        assert_eq!(&commit.to_string(), "feat(domain)!: test format");
    }

    #[test]
    fn full_conventional_commit_with_message() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            true,
            "test format".to_string(),
            Some("Message body".to_string()),
        );
        assert_eq!(
            &commit.to_string(),
            "feat(domain)!: test format\n\nMessage body"
        );
    }
}
