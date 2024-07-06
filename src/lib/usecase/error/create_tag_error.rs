use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum TagCreationError {
    RepositoryError(RepositoryError),
}

impl Display for TagCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to create tag: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for TagCreationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<Box<dyn Error>> for TagCreationError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::RepositoryError(value)
    }
}
