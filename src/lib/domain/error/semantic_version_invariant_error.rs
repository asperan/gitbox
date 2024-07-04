use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum SemanticVersionInvariantError {
    InvalidPrerelease(InvalidPrereleaseStringError),
    InvalidMetadata(InvalidMetadataStringError),
}

impl Display for SemanticVersionInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid semantic version: {}",
            self.source().expect("the source is always present")
        )
    }
}

impl Error for SemanticVersionInvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPrerelease(e) => Some(e),
            Self::InvalidMetadata(e) => Some(e),
        }
    }
}

#[derive(Debug)]
pub struct InvalidPrereleaseStringError {
    wrong_string: String,
}

impl InvalidPrereleaseStringError {
    pub fn new(wrong_string: String) -> Self {
        InvalidPrereleaseStringError { wrong_string }
    }
}

impl Display for InvalidPrereleaseStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{}' is not a valid prerelease string",
            self.wrong_string
        )
    }
}

impl Error for InvalidPrereleaseStringError {}

#[derive(Debug)]
pub struct InvalidMetadataStringError {
    wrong_string: String,
}

impl InvalidMetadataStringError {
    pub fn new(wrong_string: String) -> Self {
        InvalidMetadataStringError { wrong_string }
    }
}

impl Display for InvalidMetadataStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a valid metadata string", self.wrong_string)
    }
}

impl Error for InvalidMetadataStringError {}
