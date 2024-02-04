use std::rc::Rc;

use crate::{
    application::manager::gitextra_write_manager::GitExtraWriteManager,
    usecases::{
        repository::git_extra_egress_repository::GitExtraEgressRepository, type_aliases::AnyError,
    },
};

pub struct GitExtraWriteRepositoryImpl {
    gitextra_write_manager: Rc<dyn GitExtraWriteManager>,
}

impl GitExtraWriteRepositoryImpl {
    pub fn new(
        gitextra_write_manager: Rc<dyn GitExtraWriteManager>,
    ) -> GitExtraWriteRepositoryImpl {
        GitExtraWriteRepositoryImpl {
            gitextra_write_manager,
        }
    }
}

impl GitExtraEgressRepository for GitExtraWriteRepositoryImpl {
    fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_write_manager.update_types(types)
    }

    fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_write_manager.update_scopes(scopes)
    }
}
