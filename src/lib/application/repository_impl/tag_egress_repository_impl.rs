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
    #[test]
    fn received_label_is_version_string() {
        unimplemented!();
    }
}
