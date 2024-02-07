use std::rc::Rc;

use crate::{
    application::manager::license_text_egress_manager::LicenseTextEgressManager,
    usecase::{
        repository::license_text_egress_repository::LicenseTextEgressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseTextEgressRepositoryImpl {
    filepath: Box<str>,
    license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
}

impl LicenseTextEgressRepositoryImpl {
    pub fn new(
        filepath: &str,
        license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
    ) -> Self {
        LicenseTextEgressRepositoryImpl {
            filepath: filepath.into(),
            license_text_egress_manager,
        }
    }
}

impl LicenseTextEgressRepository for LicenseTextEgressRepositoryImpl {
    fn consume(&self, text: &str) -> Result<(), AnyError> {
        self.license_text_egress_manager
            .write_license(&self.filepath, text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        unimplemented!();
    }
}
