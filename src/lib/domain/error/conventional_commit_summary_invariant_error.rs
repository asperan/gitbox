use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ConventionalCommitSummaryInvariantError {
    Type(InvalidTypeError),
    Scope(InvalidScopeError),
    Summary(InvalidSummaryError),
}

impl Display for ConventionalCommitSummaryInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invariant violated: {}", self.source().expect("The source is always present"))
    }
}

impl Error for ConventionalCommitSummaryInvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Type(e) => Some(e),
            Self::Scope(e) => Some(e),
            Self::Summary(e) => Some(e),
        }
    }
}

impl From<InvalidTypeError> for ConventionalCommitSummaryInvariantError {
    fn from(value: InvalidTypeError) -> Self {
        Self::Type(value)
    }
}

impl From<InvalidScopeError> for ConventionalCommitSummaryInvariantError {
    fn from(value: InvalidScopeError) -> Self {
        Self::Scope(value)
    }
}

impl From<InvalidSummaryError> for ConventionalCommitSummaryInvariantError {
    fn from(value: InvalidSummaryError) -> Self {
        Self::Summary(value)
    }
}

#[derive(Debug)]
pub struct InvalidTypeError {
    wrong_type: String,
}

impl InvalidTypeError {
    pub fn new(wrong_type: String) -> Self {
        InvalidTypeError { wrong_type }
    }
}

impl Display for InvalidTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid type '{}'", self.wrong_type)
    }
}

impl Error for InvalidTypeError {}

#[derive(Debug)]
pub struct InvalidScopeError {
    wrong_scope: String,
}

impl InvalidScopeError {
    pub fn new(wrong_scope: String) -> Self {
        InvalidScopeError { wrong_scope }
    }
}

impl Display for InvalidScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid scope '{}'", self.wrong_scope)
    }
}

impl Error for InvalidScopeError {}

#[derive(Debug)]
pub struct InvalidSummaryError {
    wrong_summary: String,
}

impl InvalidSummaryError {
    pub fn new(wrong_summary: String) -> Self {
        InvalidSummaryError { wrong_summary }
    }
}

impl Display for InvalidSummaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid summary '{}'", self.wrong_summary)
    }
}

impl Error for InvalidSummaryError {}
