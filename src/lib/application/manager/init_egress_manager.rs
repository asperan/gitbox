use crate::usecase::type_aliases::AnyError;

pub trait InitEgressManager {
    fn init_repository(&self) -> Result<(), AnyError>;
}
