use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum RefreshTypesAndScopesError {
    RepositoryError(RepositoryError),
}

impl Display for RefreshTypesAndScopesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to refresh types and scopes: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for RefreshTypesAndScopesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<RepositoryError> for RefreshTypesAndScopesError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
