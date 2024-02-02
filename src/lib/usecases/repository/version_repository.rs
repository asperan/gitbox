use crate::{domain::semantic_version::SemanticVersion, usecases::type_aliases::AnyError};

pub trait SemanticVersionIngressRepository {
    fn last_version(&self) -> Result<Option<SemanticVersion>, AnyError>;
    fn last_stable_version(&self) -> Result<Option<SemanticVersion>, AnyError>;
}
