use std::rc::Rc;

use crate::{domain::semantic_version::SemanticVersion, usecase::type_aliases::AnyError};

pub trait BoundedCommitSummaryIngressManager {
    fn get_commits_from(
        &self,
        version: Rc<Option<SemanticVersion>>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = String>>, AnyError>;
}
