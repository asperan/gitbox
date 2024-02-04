use std::rc::Rc;

use crate::{
    application::retriever::commit_metadata_retriever::CommitMetadataRetriever,
    usecases::{
        metadata_spec::MetadataSpec,
        repository::commit_metadata_ingress_repository::CommitMetadataIngressRepository,
        type_aliases::AnyError,
    },
};

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

impl CommitMetadataIngressRepository for CommitMetadataRepositoryImpl {
    fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError> {
        self.commit_metadata_retriever.get_metadata(spec)
    }
}
