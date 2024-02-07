use std::rc::Rc;

use crate::{
    application::manager::gitextra_egress_manager::GitExtraEgressManager,
    usecase::{
        repository::git_extra_egress_repository::GitExtraEgressRepository, type_aliases::AnyError,
    },
};

pub struct GitExtraEgressRepositoryImpl {
    gitextra_egress_manager: Rc<dyn GitExtraEgressManager>,
}

impl GitExtraEgressRepositoryImpl {
    pub fn new(
        gitextra_egress_manager: Rc<dyn GitExtraEgressManager>,
    ) -> GitExtraEgressRepositoryImpl {
        GitExtraEgressRepositoryImpl {
            gitextra_egress_manager,
        }
    }
}

impl GitExtraEgressRepository for GitExtraEgressRepositoryImpl {
    fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_egress_manager.update_types(types)
    }

    fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        self.gitextra_egress_manager.update_scopes(scopes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn git_extra_egress_repository_impl() {
        unimplemented!();
    }
}
