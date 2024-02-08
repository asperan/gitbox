use std::rc::Rc;

use crate::{
    application::manager::tag_egress_manager::TagEgressManager,
    domain::semantic_version::SemanticVersion,
    usecase::{repository::tag_egress_repository::TagEgressRepository, type_aliases::AnyError},
};

pub struct TagEgressRepositoryImpl {
    tag_egress_manager: Rc<dyn TagEgressManager>,
}

impl TagEgressRepositoryImpl {
    pub fn new(tag_egress_manager: Rc<dyn TagEgressManager>) -> TagEgressRepositoryImpl {
        TagEgressRepositoryImpl { tag_egress_manager }
    }
}

impl TagEgressRepository for TagEgressRepositoryImpl {
    fn create_tag(
        &self,
        version: &SemanticVersion,
        message: &Option<String>,
        sign: bool,
    ) -> Result<(), AnyError> {
        self.tag_egress_manager
            .create_tag(&version.to_string(), message, sign)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        application::{
            manager::tag_egress_manager::TagEgressManager,
            repository_impl::tag_egress_repository_impl::TagEgressRepositoryImpl,
        },
        domain::semantic_version::SemanticVersion,
        usecase::{repository::tag_egress_repository::TagEgressRepository, type_aliases::AnyError},
    };

    struct MockTagEgressManager {
        label: RefCell<Box<str>>,
    }
    impl TagEgressManager for MockTagEgressManager {
        fn create_tag(
            &self,
            label: &str,
            _message: &Option<String>,
            _sign: bool,
        ) -> Result<(), AnyError> {
            self.label.replace(label.into());
            Ok(())
        }
    }

    #[test]
    fn received_label_is_version_string() {
        let version = SemanticVersion::new(0, 1, 0, None, None);
        let manager = Rc::new(MockTagEgressManager {
            label: RefCell::new("".into()),
        });
        let repository = TagEgressRepositoryImpl::new(manager.clone());
        let result = repository.create_tag(&version, &None, false);
        assert!(result.is_ok());
        assert_eq!(manager.label.borrow().as_ref(), version.to_string());
    }
}
