use std::rc::Rc;

use crate::{
    application::manager::license_list_ingress_manager::LicenseListIngressManager,
    usecases::{
        license_metadata::LicenseMetadata,
        repository::license_list_ingress_repository::LicenseListIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseListIngressRepositoryImpl {
    license_list_ingress_manager: Rc<dyn LicenseListIngressManager>,
}

impl LicenseListIngressRepositoryImpl {
    pub fn new(license_list_ingress_manager: Rc<dyn LicenseListIngressManager>) -> Self {
        LicenseListIngressRepositoryImpl {
            license_list_ingress_manager,
        }
    }
}

impl LicenseListIngressRepository for LicenseListIngressRepositoryImpl {
    fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError> {
        self.license_list_ingress_manager.license_list()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn license_list_ingress_repository_impl() {
        unimplemented!();
    }
}
