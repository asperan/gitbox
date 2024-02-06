use crate::{application::type_alias::LicenseNameAndId, usecases::type_aliases::AnyError};

pub trait LicenseListIngressManager {
    fn license_list(&self) -> Result<Box<[LicenseNameAndId]>, AnyError>;
}
