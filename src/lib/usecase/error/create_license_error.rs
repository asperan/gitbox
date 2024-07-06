use std::{error::Error, fmt::Display};

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum CreateLicenseError {
    RepositoryError(RepositoryError),
}

impl Display for CreateLicenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to create license: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for CreateLicenseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<RepositoryError> for CreateLicenseError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}
