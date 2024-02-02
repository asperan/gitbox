use crate::usecases::type_aliases::AnyError;

pub trait TagWriteManager {
    fn create_tag(&self, label: &str, message: &Option<String>, sign: bool) -> Result<(), AnyError>;
}
