use crate::{domain::semantic_version::SemanticVersion, usecase::type_aliases::AnyError};

pub trait TagEgressRepository {
    fn create_tag(
        &self,
        version: &SemanticVersion,
        message: &Option<String>,
        sign: bool,
    ) -> Result<(), AnyError>;
}
