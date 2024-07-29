use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum DataInvariantError {
    Author(AuthorInvariantError),
    Summary(SummaryInvariantError),
}

impl Display for DataInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "data invariant broken: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for DataInvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Author(err) => Some(err),
            Self::Summary(err) => Some(err),
        }
    }
}

impl From<AuthorInvariantError> for DataInvariantError {
    fn from(value: AuthorInvariantError) -> Self {
        Self::Author(value)
    }
}

impl From<SummaryInvariantError> for DataInvariantError {
    fn from(value: SummaryInvariantError) -> Self {
        Self::Summary(value)
    }
}

#[derive(Debug)]
pub enum AuthorInvariantError {
    Empty,
}

impl Display for AuthorInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid author: {}",
            match self {
                Self::Empty => "empty",
            }
        )
    }
}

impl Error for AuthorInvariantError {}

#[derive(Debug)]
pub enum SummaryInvariantError {
    Empty,
}

impl Display for SummaryInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid summary: {}",
            match self {
                Self::Empty => "empty",
            }
        )
    }
}

impl Error for SummaryInvariantError {}

#[derive(Debug)]
pub enum MetadataInvariantError {
    Hash(HashInvariantError),
    Date(DateInvariantError),
}

impl Display for MetadataInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "metadata invariant broken: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for MetadataInvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Hash(err) => Some(err),
            Self::Date(err) => Some(err),
        }
    }
}

impl From<HashInvariantError> for MetadataInvariantError {
    fn from(value: HashInvariantError) -> Self {
        Self::Hash(value)
    }
}

impl From<DateInvariantError> for MetadataInvariantError {
    fn from(value: DateInvariantError) -> Self {
        Self::Date(value)
    }
}

#[derive(Debug)]
pub enum HashInvariantError {
    Empty,
    NotHexadecimalFormat(String),
    WrongLength(usize, usize),
}

impl Display for HashInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid hash: {}",
            match self {
                Self::Empty => String::from("empty"),
                Self::NotHexadecimalFormat(s) => format!("'{}' is not a hexadecimal string", s),
                Self::WrongLength(expected, actual) =>
                    format!("wrong hash size: expected {}, got {}", expected, actual),
            }
        )
    }
}

impl Error for HashInvariantError {}

#[derive(Debug)]
pub enum DateInvariantError {
    Empty,
}

impl Display for DateInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid date: {}",
            match self {
                Self::Empty => "empty",
            }
        )
    }
}

impl Error for DateInvariantError {}
