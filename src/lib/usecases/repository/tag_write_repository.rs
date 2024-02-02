use crate::{domain::semantic_version::SemanticVersion, usecases::type_aliases::AnyError};

pub trait TagWriteRepository {
    fn create_tag(&self, version: &SemanticVersion, message: &Option<String>, sign: bool) -> Result<(), AnyError>;
}
