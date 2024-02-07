use crate::usecase::{license_metadata::LicenseMetadata, type_aliases::AnyError};

pub trait LicenseListIngressRepository {
    fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError>;
}
