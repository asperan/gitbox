use std::{error::Error, fmt::Display};

use crate::domain::error::semantic_version_invariant_error::SemanticVersionInvariantError;

#[derive(Debug)]
pub enum SemanticVersionParseError {
    InvalidMatch(SemanticVersionMatchError),
    InvalidValues(SemanticVersionInvariantError),
}

impl Display for SemanticVersionParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse semantic version: {}",
            self.source().expect("A source is always present")
        )
    }
}

impl Error for SemanticVersionParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidMatch(invalid_match) => Some(invalid_match),
            Self::InvalidValues(invalid_values) => Some(invalid_values),
        }
    }
}

impl From<SemanticVersionMatchError> for SemanticVersionParseError {
    fn from(value: SemanticVersionMatchError) -> Self {
        SemanticVersionParseError::InvalidMatch(value)
    }
}

impl From<SemanticVersionInvariantError> for SemanticVersionParseError {
    fn from(value: SemanticVersionInvariantError) -> Self {
        SemanticVersionParseError::InvalidValues(value)
    }
}

#[derive(Debug)]
pub struct SemanticVersionMatchError {
    wrong_version: String,
}

impl SemanticVersionMatchError {
    pub fn new(wrong_version: &str) -> SemanticVersionMatchError {
        SemanticVersionMatchError {
            wrong_version: wrong_version.to_string(),
        }
    }

    #[cfg(test)]
    pub fn wrong_version(&self) -> &str {
        &self.wrong_version
    }
}

impl Display for SemanticVersionMatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "version '{}' is not semantic", self.wrong_version)
    }
}

impl Error for SemanticVersionMatchError {}
