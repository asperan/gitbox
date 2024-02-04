use crate::{domain::semantic_version::SemanticVersion, usecases::type_aliases::AnyError};

pub trait BoundedCommitSummaryIngressManager {
    fn get_commits_from(
        &self,
        version: &Option<SemanticVersion>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError>;
}
