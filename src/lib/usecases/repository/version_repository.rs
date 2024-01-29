use crate::domain::semantic_version::SemanticVersion;

pub trait VersionRepository {
    fn last_version(&self) -> Option<SemanticVersion>;
    fn last_stable_version(&self) -> Option<SemanticVersion>;
}
