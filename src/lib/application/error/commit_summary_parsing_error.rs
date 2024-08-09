use std::{error::Error, fmt::Display};

use crate::domain::error::conventional_commit_summary_invariant_error::ConventionalCommitSummaryInvariantError;

#[derive(Debug)]
pub enum CommitSummaryParsingError {
    ConventionalError(ConventionalCommitSummaryInvariantError),
    FreeFormError(FreeFormCommitSummaryError),
}

impl Display for CommitSummaryParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse commit summary: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for CommitSummaryParsingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ConventionalError(err) => Some(err),
            Self::FreeFormError(err) => Some(err),
        }
    }
}

impl From<ConventionalCommitSummaryInvariantError> for CommitSummaryParsingError {
    fn from(value: ConventionalCommitSummaryInvariantError) -> Self {
        Self::ConventionalError(value)
    }
}

impl From<FreeFormCommitSummaryError> for CommitSummaryParsingError {
    fn from(value: FreeFormCommitSummaryError) -> Self {
        Self::FreeFormError(value)
    }
}

#[derive(Debug)]
pub struct FreeFormCommitSummaryError {
    message: String,
}

impl FreeFormCommitSummaryError {
    pub fn new(message: &str) -> FreeFormCommitSummaryError {
        FreeFormCommitSummaryError {
            message: message.to_owned(),
        }
    }
}

impl Display for FreeFormCommitSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse commit summary: {}", self.message)
    }
}

impl Error for FreeFormCommitSummaryError {}
