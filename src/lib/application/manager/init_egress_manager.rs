use crate::usecases::type_aliases::AnyError;

pub trait InitEgressManager {
    fn init_repository(&self) -> Result<(), AnyError>;
}
