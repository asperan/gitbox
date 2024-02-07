use std::rc::Rc;

use crate::{
    application::manager::license_choice_ingress_manager::LicenseChoiceIngressManager,
    usecases::{
        license_metadata::LicenseMetadata,
        repository::license_choice_ingress_repository::LicenseChoiceIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseChoiceIngressRepositoryImpl {
    license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>,
}

impl LicenseChoiceIngressRepositoryImpl {
    pub fn new(license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>) -> Self {
        LicenseChoiceIngressRepositoryImpl {
            license_choice_ingress_manager,
        }
    }
}

impl LicenseChoiceIngressRepository for LicenseChoiceIngressRepositoryImpl {
    fn ask_license<'a>(&self, list: &'a [LicenseMetadata]) -> Result<&'a LicenseMetadata, AnyError> {
        self.license_choice_ingress_manager.ask_license(list)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        unimplemented!();
    }
}
