use std::rc::Rc;

use crate::{
    application::manager::tag_write_manager::TagWriteManager,
    domain::semantic_version::SemanticVersion,
    usecases::{repository::tag_egress_repository::TagEgressRepository, type_aliases::AnyError},
};

pub struct TagWriteRepositoryImpl {
    tag_write_manager: Rc<dyn TagWriteManager>,
}

impl TagWriteRepositoryImpl {
    pub fn new(tag_write_manager: Rc<dyn TagWriteManager>) -> TagWriteRepositoryImpl {
        TagWriteRepositoryImpl { tag_write_manager }
    }
}

impl TagEgressRepository for TagWriteRepositoryImpl {
    fn create_tag(
        &self,
        version: &SemanticVersion,
        message: &Option<String>,
        sign: bool,
    ) -> Result<(), AnyError> {
        self.tag_write_manager
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
