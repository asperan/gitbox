use crate::usecases::{metadata_spec::MetadataSpec, type_aliases::AnyError};

pub trait CommitMetadataRetriever {
    fn get_metadata(&self, metadata_spec: &MetadataSpec) -> Result<String, AnyError>;
}
