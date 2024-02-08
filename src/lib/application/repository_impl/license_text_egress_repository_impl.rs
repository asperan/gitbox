use crate::{
    application::manager::license_text_egress_manager::LicenseTextEgressManager,
    usecase::{
        repository::license_text_egress_repository::LicenseTextEgressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseTextEgressRepositoryImpl<'a> {
    filepath: &'a str,
    license_text_egress_manager: &'a dyn LicenseTextEgressManager,
}

impl<'a, 'b: 'a, 'c: 'a> LicenseTextEgressRepositoryImpl<'a> {
    pub fn new(
        filepath: &'b str,
        license_text_egress_manager: &'c dyn LicenseTextEgressManager,
    ) -> Self {
        LicenseTextEgressRepositoryImpl {
            filepath: filepath.into(),
            license_text_egress_manager,
        }
    }
}

impl LicenseTextEgressRepository for LicenseTextEgressRepositoryImpl<'_> {
    fn consume(&self, text: &str) -> Result<(), AnyError> {
        self.license_text_egress_manager
            .write_license(&self.filepath, text)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        application::{
            manager::license_text_egress_manager::LicenseTextEgressManager,
            repository_impl::license_text_egress_repository_impl::LicenseTextEgressRepositoryImpl,
        },
        usecase::{
            repository::license_text_egress_repository::LicenseTextEgressRepository,
            type_aliases::AnyError,
        },
    };

    struct MockLicenseTextEgressManager {
        filepath: RefCell<Box<str>>,
        text: RefCell<Box<str>>,
    }
    impl LicenseTextEgressManager for MockLicenseTextEgressManager {
        fn write_license(&self, filepath: &str, text: &str) -> Result<(), AnyError> {
            self.filepath.replace(filepath.into());
            self.text.replace(text.into());
            Ok(())
        }
    }

    #[test]
    fn consume_forwards_filepath_and_text() {
        let text = "My text";
        let filepath = ".LICENSE";
        let manager = MockLicenseTextEgressManager {
            filepath: RefCell::new("".into()),
            text: RefCell::new("".into()),
        };
        let repository = LicenseTextEgressRepositoryImpl::new(filepath, &manager);
        let result = repository.consume(text);
        assert!(result.is_ok());
        assert!(
            manager.filepath.borrow().as_ref() == filepath
                && manager.text.borrow().as_ref() == text
        );
    }
}
