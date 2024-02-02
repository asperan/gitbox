use crate::usecases::type_aliases::AnyError;

pub trait GitExtraEgressRepository {
    fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError>;
    fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError>;
}
