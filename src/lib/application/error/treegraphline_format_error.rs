use std::{error::Error, fmt::Display};

use crate::domain::error::tree_graph_line_invariant_error::{
    DataInvariantError, MetadataInvariantError,
};

#[derive(Debug)]
pub enum TreeGraphLineParseError {
    LineInvariant(LineInvariantError),
    NumberOfSeparators(SeparatorNumberError),
    DataInvariant(DataInvariantError),
    MetadataInvariant(MetadataInvariantError),
}

impl Display for TreeGraphLineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse tree graph line: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for TreeGraphLineParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::LineInvariant(err) => Some(err),
            Self::NumberOfSeparators(err) => Some(err),
            Self::DataInvariant(err) => Some(err),
            Self::MetadataInvariant(err) => Some(err),
        }
    }
}

impl From<DataInvariantError> for TreeGraphLineParseError {
    fn from(value: DataInvariantError) -> Self {
        Self::DataInvariant(value)
    }
}

impl From<MetadataInvariantError> for TreeGraphLineParseError {
    fn from(value: MetadataInvariantError) -> Self {
        Self::MetadataInvariant(value)
    }
}

impl From<SeparatorNumberError> for TreeGraphLineParseError {
    fn from(value: SeparatorNumberError) -> Self {
        Self::NumberOfSeparators(value)
    }
}

impl From<LineInvariantError> for TreeGraphLineParseError {
    fn from(value: LineInvariantError) -> Self {
        Self::LineInvariant(value)
    }
}

#[derive(Debug)]
pub struct SeparatorNumberError {
    expected: usize,
    actual: usize,
}

impl SeparatorNumberError {
    pub fn new(expected: usize, actual: usize) -> Self {
        SeparatorNumberError { expected, actual }
    }
}

impl Display for SeparatorNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "wrong number of separators in line: expected {}, found {}",
            self.expected, self.actual
        )
    }
}

impl Error for SeparatorNumberError {}

#[derive(Debug)]
pub struct LineInvariantError {}

impl Display for LineInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line content error: a line cannot contain both metadata and data about a commit"
        )
    }
}

impl Error for LineInvariantError {}
