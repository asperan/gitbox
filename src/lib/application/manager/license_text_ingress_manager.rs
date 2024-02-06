use crate::{application::type_alias::LicenseNameAndId, usecases::type_aliases::AnyError};

pub trait LicenseTextIngressManager {
    fn license_text(&self, license: Box<LicenseNameAndId>) -> Result<Box<str>, AnyError>;
}
