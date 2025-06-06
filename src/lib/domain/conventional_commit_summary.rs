use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Deref;

use super::error::conventional_commit_summary_invariant_error::{
    ConventionalCommitSummaryInvariantError, InvalidScopeError, InvalidSummaryError,
    InvalidTypeError,
};

lazy_static! {
    static ref SCOPE_PATTERN: Regex = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
    static ref TYPE_PATTERN: Regex = Regex::new(r"^[a-z]+$").unwrap();
}

/*
Summary of a conventional commit.

It contains the type, the optional scope, the breaking-ness
and the first line of the commit message.

For more information about the specification of a conventional commit,
see [the official documentation](https://www.conventionalcommits.org/en/v1.0.0/).

If the complete message is needed, see [ConventionalCommit].
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionalCommitSummary {
    typ: String,
    scope: Option<String>,
    breaking: ConventionalCommitSummaryBreakingFlag,
    summary: String,
}

impl ConventionalCommitSummary {
    pub fn new(
        typ: String,
        scope: Option<String>,
        breaking: ConventionalCommitSummaryBreakingFlag,
        summary: String,
    ) -> Result<Self, ConventionalCommitSummaryInvariantError> {
        Ok(ConventionalCommitSummary {
            typ: Self::check_type(typ)?,
            scope: Self::check_scope(scope)?,
            breaking,
            summary: Self::check_summary(summary)?,
        })
    }

    pub fn typ(&self) -> &str {
        &self.typ
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn breaking(&self) -> bool {
        *self.breaking
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    fn check_type(typ: String) -> Result<String, InvalidTypeError> {
        if TYPE_PATTERN.is_match(&typ) {
            Ok(typ)
        } else {
            Err(InvalidTypeError::new(typ))
        }
    }

    fn check_scope(scope: Option<String>) -> Result<Option<String>, InvalidScopeError> {
        match scope {
            None => Ok(None),
            Some(s) => {
                if SCOPE_PATTERN.is_match(&s) {
                    Ok(Some(s))
                } else {
                    Err(InvalidScopeError::new(s))
                }
            }
        }
    }

    fn check_summary(summary: String) -> Result<String, InvalidSummaryError> {
        if summary.trim().is_empty() {
            Err(InvalidSummaryError::new(summary))
        } else {
            Ok(summary)
        }
    }

    pub fn type_pattern() -> &'static Regex {
        &TYPE_PATTERN
    }

    pub fn scope_pattern() -> &'static Regex {
        &SCOPE_PATTERN
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConventionalCommitSummaryBreakingFlag {
    Enabled,
    Disabled,
}

impl Deref for ConventionalCommitSummaryBreakingFlag {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Enabled => &true,
            Self::Disabled => &false,
        }
    }
}

impl AsRef<bool> for ConventionalCommitSummaryBreakingFlag {
    fn as_ref(&self) -> &bool {
        self.deref()
    }
}

impl From<bool> for ConventionalCommitSummaryBreakingFlag {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::domain::conventional_commit_summary::{
        ConventionalCommitSummary, ConventionalCommitSummaryBreakingFlag,
    };

    #[test]
    fn type_wrong_invariants() {
        let test_string1_empty = String::from("");
        let test_string2_only_spaces = String::from("  ");
        let test_string3_uppercase = String::from("FEAT");
        let test_string4_hypen = String::from("test-4");
        assert!(ConventionalCommitSummary::check_type(test_string1_empty).is_err());
        assert!(ConventionalCommitSummary::check_type(test_string2_only_spaces).is_err());
        assert!(ConventionalCommitSummary::check_type(test_string3_uppercase).is_err());
        assert!(ConventionalCommitSummary::check_type(test_string4_hypen).is_err());
    }

    #[test]
    fn type_correct_invariants() {
        let correct_type = String::from("feat");
        assert!(ConventionalCommitSummary::check_type(correct_type).is_ok());
    }

    #[test]
    fn scope_wrong_invariants() {
        let test_scope1_empty = Some(String::from(""));
        let test_scope2_only_spaces = Some(String::from("  "));
        let test_scope3_symbols = Some(String::from("test/what"));
        assert!(ConventionalCommitSummary::check_scope(test_scope1_empty).is_err());
        assert!(ConventionalCommitSummary::check_scope(test_scope2_only_spaces).is_err());
        assert!(ConventionalCommitSummary::check_scope(test_scope3_symbols).is_err());
    }

    #[test]
    fn scope_correct_invariants() {
        let correct_scope1_none = None;
        let correct_scope2_lowercase = Some(String::from("api"));
        let correct_scope3_uppercase = Some(String::from("API"));
        let correct_scope4_hypen = Some(String::from("test-api"));
        assert!(ConventionalCommitSummary::check_scope(correct_scope1_none).is_ok());
        assert!(ConventionalCommitSummary::check_scope(correct_scope2_lowercase).is_ok());
        assert!(ConventionalCommitSummary::check_scope(correct_scope3_uppercase).is_ok());
        assert!(ConventionalCommitSummary::check_scope(correct_scope4_hypen).is_ok());
    }

    #[test]
    fn summary_wrong_invariants() {
        let wrong_summary1_empty = String::from("");
        let wrong_summary2_onyl_spaces = String::from("  ");
        assert!(ConventionalCommitSummary::check_summary(wrong_summary1_empty).is_err());
        assert!(ConventionalCommitSummary::check_summary(wrong_summary2_onyl_spaces).is_err());
    }

    #[test]
    fn summary_correct_invariants() {
        let correct_summary1 =
            String::from("a test Summary with all the available characters - even symbols");
        assert!(ConventionalCommitSummary::check_summary(correct_summary1).is_ok());
    }

    #[test]
    fn flag_enabled() {
        assert!(ConventionalCommitSummaryBreakingFlag::Enabled.deref());
    }

    #[test]
    fn flag_disabled() {
        assert!(!ConventionalCommitSummaryBreakingFlag::Disabled.deref());
    }
}
