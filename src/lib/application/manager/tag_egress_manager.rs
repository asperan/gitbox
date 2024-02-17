use crate::usecase::type_aliases::AnyError;

pub trait TagEgressManager {
    fn create_tag(&self, label: &str, message: Option<&str>, sign: bool) -> Result<(), AnyError>;
}
