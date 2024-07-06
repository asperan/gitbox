use std::{error::Error, fmt::Display};

use crate::domain::error::conventional_commit_error::ConventionalCommitError;

// The type of the Box value should be more specific, but it cannot as trait upcast is not
// permitted yet
type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum CreateConventionalCommitError {
    CreationError(ConventionalCommitError),
    RepositoryError(RepositoryError),
}

impl Display for CreateConventionalCommitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to create conventional commit: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for CreateConventionalCommitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CreationError(err) => Some(err),
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<ConventionalCommitError> for CreateConventionalCommitError {
    fn from(value: ConventionalCommitError) -> Self {
        Self::CreationError(value)
    }
}

impl From<RepositoryError> for CreateConventionalCommitError {
    fn from(value: Box<(dyn std::error::Error + 'static)>) -> Self {
        Self::RepositoryError(value)
    }
}
