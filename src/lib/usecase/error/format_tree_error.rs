use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum FormatTreeError {
    NoCommits(NoCommitsError),
    RepositoryError(RepositoryError),
}

impl Display for FormatTreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to format tree: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for FormatTreeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoCommits(err) => Some(err),
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<RepositoryError> for FormatTreeError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

impl From<NoCommitsError> for FormatTreeError {
    fn from(value: NoCommitsError) -> Self {
        Self::NoCommits(value)
    }
}

#[derive(Debug)]
pub struct NoCommitsError {}

impl NoCommitsError {
    pub fn new() -> Self {
        NoCommitsError {}
    }
}

impl Display for NoCommitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot create tree with no commits")
    }
}

impl Error for NoCommitsError {}
