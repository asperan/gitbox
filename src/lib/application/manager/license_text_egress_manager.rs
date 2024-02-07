use crate::usecase::type_aliases::AnyError;

pub trait LicenseTextEgressManager {
    fn write_license(&self, filepath: &str, text: &str) -> Result<(), AnyError>;
}
