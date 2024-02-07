use std::rc::Rc;

use crate::{
    application::manager::license_text_ingress_manager::LicenseTextIngressManager,
    usecase::{
        license_metadata::LicenseMetadata,
        repository::license_text_ingress_repository::LicenseTextIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseTextIngressRepositoryImpl {
    license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>,
}

impl LicenseTextIngressRepositoryImpl {
    pub fn new(license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>) -> Self {
        LicenseTextIngressRepositoryImpl {
            license_text_ingress_manager,
        }
    }
}

impl LicenseTextIngressRepository for LicenseTextIngressRepositoryImpl {
    fn text(&self, license: &LicenseMetadata) -> Result<Box<str>, AnyError> {
        self.license_text_ingress_manager.license_text(license)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        unimplemented!();
    }
}
