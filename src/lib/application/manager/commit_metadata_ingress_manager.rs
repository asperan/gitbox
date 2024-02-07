use crate::usecase::{metadata_spec::MetadataSpec, type_aliases::AnyError};

pub trait CommitMetadataIngressManager {
    fn get_metadata(&self, metadata_spec: &MetadataSpec) -> Result<String, AnyError>;
}
