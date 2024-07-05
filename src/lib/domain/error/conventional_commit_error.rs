use std::{error::Error, fmt::Display};

use super::conventional_commit_summary_invariant_error::ConventionalCommitSummaryInvariantError;

#[derive(Debug)]
pub enum ConventionalCommitError {
    SummaryError(ConventionalCommitSummaryInvariantError),
}

impl Display for ConventionalCommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to create conventional commit: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for ConventionalCommitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SummaryError(err) => Some(err),
        }
    }
}

impl From<ConventionalCommitSummaryInvariantError> for ConventionalCommitError {
    fn from(value: ConventionalCommitSummaryInvariantError) -> Self {
        Self::SummaryError(value)
    }
}
