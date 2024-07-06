use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum ChangelogCreationError {
    RepositoryError(RepositoryError),
}

impl Display for ChangelogCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to create changelog: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for ChangelogCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<Box<dyn Error>> for ChangelogCreationError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::RepositoryError(value)
    }
}
