use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum FormatTreeError {
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
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<RepositoryError> for FormatTreeError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
