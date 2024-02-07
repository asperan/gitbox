use crate::usecases::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseChoiceIngressManager {
    fn ask_license<'a>(&self, list: &'a[LicenseMetadata]) -> Result<&'a LicenseMetadata, AnyError>;
}
