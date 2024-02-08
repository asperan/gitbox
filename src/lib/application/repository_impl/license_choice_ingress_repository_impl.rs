use std::rc::Rc;

use crate::{
    application::manager::license_choice_ingress_manager::LicenseChoiceIngressManager,
    usecase::{
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
    fn ask_license<'a>(
        &self,
        list: &'a [LicenseMetadata],
    ) -> Result<&'a LicenseMetadata, AnyError> {
        self.license_choice_ingress_manager.ask_license(list)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{manager::license_choice_ingress_manager::LicenseChoiceIngressManager, repository_impl::license_choice_ingress_repository_impl::LicenseChoiceIngressRepositoryImpl},
        usecase::{license_metadata::LicenseMetadata, repository::license_choice_ingress_repository::LicenseChoiceIngressRepository, type_aliases::AnyError},
    };

    #[derive(Debug)]
    struct MockError {}
    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }
    impl Error for MockError {}

    struct MockLicenseChoiceIngressManager {}

    impl LicenseChoiceIngressManager for MockLicenseChoiceIngressManager {
        fn ask_license<'a>(
            &self,
            list: &'a [LicenseMetadata],
        ) -> Result<&'a LicenseMetadata, AnyError> {
            if !list.is_empty() {
                Ok(&list[0])
            } else {
                Err(MockError{}.into())
            }
        }
    }

    #[test]
    fn ask_license_ok() {
        let choice_list = [LicenseMetadata::new("MIT", "mit-license"), LicenseMetadata::new("MPL v2", "mpl-license")];
        let license_choice_ingress_manager = Rc::new(MockLicenseChoiceIngressManager{});
        let repository = LicenseChoiceIngressRepositoryImpl::new(license_choice_ingress_manager.clone());
        let result = repository.ask_license(&choice_list);
        assert!(result.is_ok());
    }
}
