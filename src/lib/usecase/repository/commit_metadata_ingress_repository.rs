use crate::usecase::{metadata_spec::MetadataSpec, type_aliases::AnyError};

pub trait CommitMetadataIngressRepository {
    fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError>;
}
