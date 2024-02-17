use std::rc::Rc;

use crate::{
    domain::{commit_summary::CommitSummary, semantic_version::SemanticVersion},
    usecase::type_aliases::AnyError,
};

pub trait BoundedCommitSummaryIngressRepository {
    fn get_commits_from(
        &self,
        version: Rc<Option<SemanticVersion>>,
    ) -> Result<Box<dyn DoubleEndedIterator<Item = CommitSummary>>, AnyError>;
}
