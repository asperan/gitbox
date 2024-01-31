use crate::domain::type_aliases::AnyError;

pub trait InitManager {
    fn init_repository(&self) -> Result<(), AnyError>;
}
