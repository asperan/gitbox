use std::rc::Rc;

use crate::{
    application::retriever::commit_metadata_retriever::CommitMetadataIngressManager,
    usecases::{
        metadata_spec::MetadataSpec,
        repository::commit_metadata_ingress_repository::CommitMetadataIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct CommitMetadataIngressRepositoryImpl {
    commit_metadata_retriever: Rc<dyn CommitMetadataIngressManager>,
}

impl CommitMetadataIngressRepositoryImpl {
    pub fn new(
        commit_metadata_retriever: Rc<dyn CommitMetadataIngressManager>,
    ) -> CommitMetadataIngressRepositoryImpl {
        CommitMetadataIngressRepositoryImpl {
            commit_metadata_retriever,
        }
    }
}

impl CommitMetadataIngressRepository for CommitMetadataIngressRepositoryImpl {
    fn get_metadata(&self, spec: &MetadataSpec) -> Result<String, AnyError> {
        self.commit_metadata_retriever.get_metadata(spec)
    }
}
