use crate::usecase::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseChoiceIngressRepository {
    fn ask_license<'a>(&self, list: &'a [LicenseMetadata])
        -> Result<&'a LicenseMetadata, AnyError>;
}
