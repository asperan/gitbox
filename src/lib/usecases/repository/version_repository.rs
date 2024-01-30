use crate::domain::{semantic_version::SemanticVersion, type_aliases::AnyError};

pub trait VersionRepository {
    fn last_version(&self) -> Result<Option<SemanticVersion>, AnyError>;
    fn last_stable_version(&self) -> Result<Option<SemanticVersion>, AnyError>;
}
