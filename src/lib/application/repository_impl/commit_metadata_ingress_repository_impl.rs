use std::rc::Rc;

use crate::{
    application::manager::commit_metadata_ingress_manager::CommitMetadataIngressManager,
    usecase::{
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
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            manager::commit_metadata_ingress_manager::CommitMetadataIngressManager,
            repository_impl::commit_metadata_ingress_repository_impl::CommitMetadataIngressRepositoryImpl,
        },
        usecase::{
            metadata_spec::MetadataSpec,
            repository::commit_metadata_ingress_repository::CommitMetadataIngressRepository,
            type_aliases::AnyError,
        },
    };

    #[derive(Debug)]
    struct MockError {}
    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }
    impl Error for MockError {}

    struct MockCommitMetadataIngressManager {
        fail: bool,
    }

    impl CommitMetadataIngressManager for MockCommitMetadataIngressManager {
        fn get_metadata(&self, _metadata_spec: &MetadataSpec) -> Result<String, AnyError> {
            if self.fail {
                Err(MockError {}.into())
            } else {
                Ok("metadata".to_string())
            }
        }
    }

    #[test]
    fn get_metadata_ok() {
        let commit_metadata_ingress_manager =
            Rc::new(MockCommitMetadataIngressManager { fail: false });
        let repository_impl =
            CommitMetadataIngressRepositoryImpl::new(commit_metadata_ingress_manager.clone());
        let result = repository_impl.get_metadata(&MetadataSpec::Sha);
        assert!(result.is_ok_and(|it| it == "metadata"));
    }

    #[test]
    fn get_metadata_err() {
        let commit_metadata_ingress_manager =
            Rc::new(MockCommitMetadataIngressManager { fail: true });
        let repository_impl =
            CommitMetadataIngressRepositoryImpl::new(commit_metadata_ingress_manager.clone());
        let result = repository_impl.get_metadata(&MetadataSpec::Sha);
        assert!(result.is_err());
    }
}
