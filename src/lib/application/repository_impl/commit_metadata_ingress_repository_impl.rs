use std::rc::Rc;

use crate::{
    application::manager::commit_metadata_ingress_manager::CommitMetadataIngressManager,
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

#[cfg(test)]
mod tests {
    #[test]
    fn commit_metadata_ingress_repository_impl() {
        unimplemented!();
    }
}
