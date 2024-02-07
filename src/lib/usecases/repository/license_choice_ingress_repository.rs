use crate::usecases::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseChoiceIngressRepository {
    fn ask_license(&self, list: &[LicenseMetadata]) -> Result<&LicenseMetadata, AnyError>;
}
