use crate::usecase::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseListIngressManager {
    fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError>;
}
