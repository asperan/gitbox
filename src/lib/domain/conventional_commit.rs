use super::{
    conventional_commit_summary::{
        ConventionalCommitSummary, ConventionalCommitSummaryBreakingFlag,
    },
    error::conventional_commit_error::ConventionalCommitError,
};

/**
A [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/).

Other than a [ConventionalCommitSummary], it also contains the optional message.

This message should contain all the lines but the first,
so no duplicate information is present.
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConventionalCommit {
    summary: ConventionalCommitSummary,
    message: Option<String>,
}

impl ConventionalCommit {
    pub fn new(
        typ: String,
        scope: Option<String>,
        breaking: ConventionalCommitSummaryBreakingFlag,
        summary: String,
        message: Option<String>,
    ) -> Result<Self, ConventionalCommitError> {
        Ok(ConventionalCommit {
            summary: ConventionalCommitSummary::new(typ, scope, breaking, summary)?,
            message,
        })
    }

    pub fn summary(&self) -> &ConventionalCommitSummary {
        &self.summary
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
