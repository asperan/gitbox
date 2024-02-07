use crate::usecases::type_aliases::AnyError;

pub trait LicenseTextEgressRepository {
    fn consume(&self, text: &str) -> Result<(), AnyError>;
}
