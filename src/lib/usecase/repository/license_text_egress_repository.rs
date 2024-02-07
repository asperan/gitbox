use crate::usecase::type_aliases::AnyError;

pub trait LicenseTextEgressRepository {
    fn consume(&self, text: &str) -> Result<(), AnyError>;
}
