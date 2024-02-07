use crate::usecases::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseTextIngressManager {
    fn license_text(&self, license: &LicenseMetadata) -> Result<Box<str>, AnyError>;
}
