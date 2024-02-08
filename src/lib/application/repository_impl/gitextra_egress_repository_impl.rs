use crate::{
    application::manager::gitextra_egress_manager::GitExtraEgressManager,
    usecase::{
        repository::git_extra_egress_repository::GitExtraEgressRepository, type_aliases::AnyError,
    },
};

pub struct GitExtraEgressRepositoryImpl<'a> {
    gitextra_egress_manager: &'a dyn GitExtraEgressManager,
}

impl<'a, 'b: 'a> GitExtraEgressRepositoryImpl<'a> {
    pub fn new(
        gitextra_egress_manager: &'b dyn GitExtraEgressManager,
    ) -> Self {
        GitExtraEgressRepositoryImpl {
            gitextra_egress_manager,
        }
    }
}

impl GitExtraEgressRepository for GitExtraEgressRepositoryImpl<'_> {
    fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_egress_manager.update_types(types)
    }

    fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_egress_manager.update_scopes(scopes)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, fmt::Display};

    use crate::{
        application::{
            manager::gitextra_egress_manager::GitExtraEgressManager,
            repository_impl::gitextra_egress_repository_impl::GitExtraEgressRepositoryImpl,
        },
        usecase::{
            repository::git_extra_egress_repository::GitExtraEgressRepository,
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

    struct MockGitExtraEgressManager {
        fail: bool,
        types: RefCell<Vec<String>>,
        scopes: RefCell<Vec<String>>,
    }

    impl GitExtraEgressManager for MockGitExtraEgressManager {
        fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            if self.fail {
                Err(MockError {}.into())
            } else {
                self.types.replace(types.collect());
                Ok(())
            }
        }
        fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
            if self.fail {
                Err(MockError {}.into())
            } else {
                self.scopes.replace(scopes.collect());
                Ok(())
            }
        }
    }

    #[test]
    fn update_types_ok() {
        let types = ["type1".to_string(), "type2".to_string()];
        let git_extra_egress_manager = MockGitExtraEgressManager {
            fail: false,
            types: RefCell::new(vec![]),
            scopes: RefCell::new(vec![]),
        };
        let repository = GitExtraEgressRepositoryImpl::new(&git_extra_egress_manager);
        let result = repository.update_types(Box::new(types.clone().into_iter()));
        assert!(result.is_ok() && git_extra_egress_manager.types.borrow().as_slice() == &types);
    }

    #[test]
    fn update_types_err() {
        let types = ["type1".to_string(), "type2".to_string()];
        let git_extra_egress_manager = MockGitExtraEgressManager {
            fail: true,
            types: RefCell::new(vec![]),
            scopes: RefCell::new(vec![]),
        };
        let repository = GitExtraEgressRepositoryImpl::new(&git_extra_egress_manager);
        let result = repository.update_types(Box::new(types.clone().into_iter()));
        assert!(result.is_err());
    }

    #[test]
    fn update_scopes_ok() {
        let scopes = ["scope1".to_string(), "scope2".to_string()];
        let git_extra_egress_manager = MockGitExtraEgressManager {
            fail: false,
            types: RefCell::new(vec![]),
            scopes: RefCell::new(vec![]),
        };
        let repository = GitExtraEgressRepositoryImpl::new(&git_extra_egress_manager);
        let result = repository.update_scopes(Box::new(scopes.clone().into_iter()));
        assert!(result.is_ok() && git_extra_egress_manager.scopes.borrow().as_slice() == &scopes);
    }

    #[test]
    fn update_scopes_err() {
        let scopes = ["scope1".to_string(), "scope2".to_string()];
        let git_extra_egress_manager = MockGitExtraEgressManager {
            fail: true,
            types: RefCell::new(vec![]),
            scopes: RefCell::new(vec![]),
        };
        let repository = GitExtraEgressRepositoryImpl::new(&git_extra_egress_manager);
        let result = repository.update_scopes(Box::new(scopes.clone().into_iter()));
        assert!(result.is_err());
    }
}
