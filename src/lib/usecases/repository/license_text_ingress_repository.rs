use crate::usecases::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseTextIngressRepository {
    fn text(&self, license: &LicenseMetadata) -> Result<Box<str>, AnyError>;
}
