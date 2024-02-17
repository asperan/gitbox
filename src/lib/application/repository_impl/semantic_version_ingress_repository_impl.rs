use std::{rc::Rc, str::FromStr};

use crate::{
    application::manager::version_ingress_manager::VersionIngressManager,
    domain::semantic_version::SemanticVersion,
    usecase::{
        repository::semantic_version_ingress_repository::SemanticVersionIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct SemanticVersionIngressRepositoryImpl<'a> {
    version_ingress_manager: &'a dyn VersionIngressManager,
}

impl<'a, 'b: 'a> SemanticVersionIngressRepositoryImpl<'a> {
    pub fn new(version_ingress_manager: &'b dyn VersionIngressManager) -> Self {
        SemanticVersionIngressRepositoryImpl {
            version_ingress_manager,
        }
    }
}

impl SemanticVersionIngressRepository for SemanticVersionIngressRepositoryImpl<'_> {
    fn last_version(&self) -> Result<Rc<Option<SemanticVersion>>, AnyError> {
        let version = self.version_ingress_manager.last_version()?;
        Ok(match version {
            Some(s) => Some(SemanticVersion::from_str(&s)?),
            None => None,
        }.into())
    }

    fn last_stable_version(&self) -> Result<Rc<Option<SemanticVersion>>, AnyError> {
        let version = self.version_ingress_manager.last_stable_version()?;
        Ok(match version {
            Some(v) => Some(SemanticVersion::from_str(&v)?),
            None => None,
        }.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            manager::version_ingress_manager::VersionIngressManager,
            repository_impl::semantic_version_ingress_repository_impl::SemanticVersionIngressRepositoryImpl,
        },
        domain::semantic_version::SemanticVersion,
        usecase::{
            repository::semantic_version_ingress_repository::SemanticVersionIngressRepository,
            type_aliases::AnyError,
        },
    };

    struct MockEmptyVersionRetriever {}

    impl VersionIngressManager for MockEmptyVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }
    }

    struct MockFullVersionRetriever {}

    impl VersionIngressManager for MockFullVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("0.1.0-dev1")))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("0.1.0")))
        }
    }

    struct MockWrongVersionRetriever {}

    impl VersionIngressManager for MockWrongVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("22.04")))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("22-04-12")))
        }
    }

    #[test]
    fn last_version_present() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockFullVersionRetriever {});
        let expected = SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), None);
        assert!(repository
            .last_version()
            .is_ok_and(|it| it.as_ref().clone().is_some_and(|v| v == expected)));
    }

    #[test]
    fn last_version_empty() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockEmptyVersionRetriever {});
        assert!(repository.last_version().is_ok_and(|it| it.is_none()));
    }

    #[test]
    fn last_version_wrong() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockWrongVersionRetriever {});
        assert!(repository.last_version().is_err());
    }

    #[test]
    fn last_stable_version_present() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockFullVersionRetriever {});
        let expected = SemanticVersion::new(0, 1, 0, None, None);
        assert!(repository
            .last_stable_version()
            .is_ok_and(|it| it.as_ref().clone().is_some_and(|v| v == expected)));
    }

    #[test]
    fn last_stable_version_empty() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockEmptyVersionRetriever {});
        assert!(repository
            .last_stable_version()
            .is_ok_and(|it| it.is_none()));
    }

    #[test]
    fn last_stable_version_wrong() {
        let repository = SemanticVersionIngressRepositoryImpl::new(&MockWrongVersionRetriever {});
        assert!(repository.last_stable_version().is_err());
    }
}
