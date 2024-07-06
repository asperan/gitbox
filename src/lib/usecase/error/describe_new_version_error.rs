use std::{error::Error, fmt::Display};

use crate::domain::error::semantic_version_invariant_error::SemanticVersionInvariantError;

use super::describe_no_relevant_changes_error::DescribeNoRelevantChangesError;

type RepositoryError = Box<dyn Error>;

#[derive(Debug)]
pub enum DescribeNewVersionError {
    StableReleaseError(DescribeStableReleaseError),
    PrereleaseError(DescribePrereleaseError),
    MetadataError(DescribeMetadataError),
    SemanticVersionCreationError(SemanticVersionInvariantError),
    RepositoryError(RepositoryError),
}

impl Display for DescribeNewVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to describe new version: {}",
            self.source().expect("source error is always present")
        )
    }
}

impl Error for DescribeNewVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::SemanticVersionCreationError(err) => Some(err),
            Self::StableReleaseError(err) => Some(err),
            Self::PrereleaseError(err) => Some(err),
            Self::MetadataError(err) => Some(err),
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<DescribeStableReleaseError> for DescribeNewVersionError {
    fn from(value: DescribeStableReleaseError) -> Self {
        Self::StableReleaseError(value)
    }
}

impl From<DescribePrereleaseError> for DescribeNewVersionError {
    fn from(value: DescribePrereleaseError) -> Self {
        Self::PrereleaseError(value)
    }
}

impl From<DescribeMetadataError> for DescribeNewVersionError {
    fn from(value: DescribeMetadataError) -> Self {
        Self::MetadataError(value)
    }
}

impl From<SemanticVersionInvariantError> for DescribeNewVersionError {
    fn from(value: SemanticVersionInvariantError) -> Self {
        Self::SemanticVersionCreationError(value)
    }
}

impl From<RepositoryError> for DescribeNewVersionError {
    fn from(value: RepositoryError) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum DescribeStableReleaseError {
    NoChanges(DescribeNoRelevantChangesError),
    RepositoryError(RepositoryError),
}

impl Display for DescribeStableReleaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to describe stable release: {}", self.source().expect("source error is always present"))
    }
}

impl Error for DescribeStableReleaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoChanges(err) => Some(err),
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<DescribeNoRelevantChangesError> for DescribeStableReleaseError {
    fn from(value: DescribeNoRelevantChangesError) -> Self {
        Self::NoChanges(value)
    }
}

impl From<Box<dyn Error>> for DescribeStableReleaseError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum DescribePrereleaseError {
    NoChanges(DescribeNoRelevantChangesError),
    RepositoryError(RepositoryError),
}

impl Display for DescribePrereleaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to describe prerelease: {}", self.source().expect("source error is always present"))
    }
}

impl Error for DescribePrereleaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoChanges(err) => Some(err),
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<DescribeNoRelevantChangesError> for DescribePrereleaseError {
    fn from(value: DescribeNoRelevantChangesError) -> Self {
        Self::NoChanges(value)
    }
}

impl From<Box<dyn Error>> for DescribePrereleaseError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::RepositoryError(value)
    }
}

#[derive(Debug)]
pub enum DescribeMetadataError {
    RepositoryError(RepositoryError),
}

impl Display for DescribeMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to describe metadata: {}", self.source().expect("source error is always present"))
    }
}

impl Error for DescribeMetadataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
        }
    }
}

impl From<Box<dyn Error>> for DescribeMetadataError {
    fn from(value: Box<dyn Error>) -> Self {
        Self::RepositoryError(value)
    }
}
