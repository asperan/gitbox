use crate::{
    application::manager::license_text_ingress_manager::LicenseTextIngressManager,
    usecase::{
        license_metadata::LicenseMetadata,
        repository::license_text_ingress_repository::LicenseTextIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseTextIngressRepositoryImpl<'a> {
    license_text_ingress_manager: &'a dyn LicenseTextIngressManager,
}

impl<'a, 'b: 'a> LicenseTextIngressRepositoryImpl<'a> {
    pub fn new(license_text_ingress_manager: &'b dyn LicenseTextIngressManager) -> Self {
        LicenseTextIngressRepositoryImpl {
            license_text_ingress_manager,
        }
    }
}

impl LicenseTextIngressRepository for LicenseTextIngressRepositoryImpl<'_> {
    fn text(&self, license: &LicenseMetadata) -> Result<Box<str>, AnyError> {
        self.license_text_ingress_manager.license_text(license)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            manager::license_text_ingress_manager::LicenseTextIngressManager,
            repository_impl::license_text_ingress_repository_impl::LicenseTextIngressRepositoryImpl,
        },
        usecase::{
            license_metadata::LicenseMetadata,
            repository::license_text_ingress_repository::LicenseTextIngressRepository,
            type_aliases::AnyError,
        },
    };

    struct MockLicenseTextIngressManager {
        text: Box<str>,
    }
    impl LicenseTextIngressManager for MockLicenseTextIngressManager {
        fn license_text(&self, _license: &LicenseMetadata) -> Result<Box<str>, AnyError> {
            Ok(self.text.clone())
        }
    }

    #[test]
    fn license_text() {
        let text = "License test text";
        let manager = MockLicenseTextIngressManager { text: text.into() };
        let repository = LicenseTextIngressRepositoryImpl::new(&manager);
        let result = repository.text(&LicenseMetadata::new("Stub", "stub"));
        assert!(result.is_ok());
        assert_eq!(result.expect("Just asserted the OK-ness"), text.into());
    }
}
