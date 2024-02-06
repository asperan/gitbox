use crate::{application::type_alias::LicenseNameAndId, usecases::type_aliases::AnyError};

pub trait LicenseChoiceIngressManager {
    fn ask_license(&self, list: Box<[LicenseNameAndId]>)
        -> Result<Box<LicenseNameAndId>, AnyError>;
}
