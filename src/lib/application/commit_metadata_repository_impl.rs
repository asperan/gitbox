use std::rc::Rc;

use crate::usecases::{
    metadata_spec::MetadataSpec, repository::commit_metadata_repository::CommitMetadataRepository,
    type_aliases::AnyError,
};

use super::retriever::commit_metadata_retriever::CommitMetadataRetriever;

pub struct CommitMetadataRepositoryImpl {
    commit_metadata_retriever: Rc<dyn CommitMetadataRetriever>,
}

impl CommitMetadataRepositoryImpl {
    pub fn new(
        commit_metadata_retriever: Rc<dyn CommitMetadataRetriever>,
    ) -> CommitMetadataRepositoryImpl {
        CommitMetadataRepositoryImpl {
            commit_metadata_retriever,
        }
    }
}

impl CommitMetadataRepository for CommitMetadataRepositoryImpl {
    fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError> {
        self.commit_metadata_retriever.get_metadata(spec)
    }
}
