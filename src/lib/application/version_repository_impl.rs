use std::{rc::Rc, str::FromStr};

use crate::{
    domain::{semantic_version::SemanticVersion, type_aliases::AnyError},
    usecases::repository::version_repository::VersionRepository,
};

use super::retriever::version_retriever::VersionRetriever;

pub struct VersionRepositoryImpl {
    version_retriever: Rc<dyn VersionRetriever>,
}

impl VersionRepositoryImpl {
    pub fn new(version_retriever: Rc<dyn VersionRetriever>) -> VersionRepositoryImpl {
        VersionRepositoryImpl { version_retriever }
    }
}

impl VersionRepository for VersionRepositoryImpl {
    fn last_version(&self) -> Result<Option<SemanticVersion>, AnyError> {
        let version = self.version_retriever.last_version()?;
        Ok(match version {
            Some(s) => Some(SemanticVersion::from_str(&s)?),
            None => None,
        })
    }

    fn last_stable_version(&self) -> Result<Option<SemanticVersion>, AnyError> {
        let version = self.version_retriever.last_stable_version()?;
        Ok(match version {
            Some(v) => Some(SemanticVersion::from_str(&v)?),
            None => None,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        application::{
            retriever::version_retriever::VersionRetriever,
            version_repository_impl::VersionRepositoryImpl,
        },
        domain::{semantic_version::SemanticVersion, type_aliases::AnyError},
        usecases::repository::version_repository::VersionRepository,
    };

    struct MockEmptyVersionRetriever {}

    impl VersionRetriever for MockEmptyVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(None)
        }
    }

    struct MockFullVersionRetriever {}

    impl VersionRetriever for MockFullVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("0.1.0-dev1")))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("0.1.0")))
        }
    }

    struct MockWrongVersionRetriever {}

    impl VersionRetriever for MockWrongVersionRetriever {
        fn last_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("22.04")))
        }

        fn last_stable_version(&self) -> Result<Option<String>, AnyError> {
            Ok(Some(String::from("22-04-12")))
        }
    }

    #[test]
    fn last_version_present() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockFullVersionRetriever {}));
        let expected = SemanticVersion::new(0, 1, 0, Some("dev1".to_string()), None);
        assert!(repository
            .last_version()
            .is_ok_and(|it| it.is_some_and(|v| v == expected)));
    }

    #[test]
    fn last_version_empty() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockEmptyVersionRetriever {}));
        assert!(repository.last_version().is_ok_and(|it| it.is_none()));
    }

    #[test]
    fn last_version_wrong() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockWrongVersionRetriever {}));
        assert!(repository.last_version().is_err());
    }

    #[test]
    fn last_stable_version_present() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockFullVersionRetriever {}));
        let expected = SemanticVersion::new(0, 1, 0, None, None);
        assert!(repository
            .last_stable_version()
            .is_ok_and(|it| it.is_some_and(|v| v == expected)));
    }

    #[test]
    fn last_stable_version_empty() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockEmptyVersionRetriever {}));
        assert!(repository
            .last_stable_version()
            .is_ok_and(|it| it.is_none()));
    }

    #[test]
    fn last_stable_version_wrong() {
        let repository = VersionRepositoryImpl::new(Rc::new(MockWrongVersionRetriever {}));
        assert!(repository.last_stable_version().is_err());
    }
}
