use crate::usecases::type_aliases::AnyError;

pub trait LicenseTextEgressManager {
    fn write_license(&self, filepath: &str, text: &str) -> Result<(), AnyError>;
}