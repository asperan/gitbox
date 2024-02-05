use crate::usecases::type_aliases::AnyError;

pub trait GitExtraEgressHelper {
    fn append_type(&self, new_type: &str) -> Result<(), AnyError>;
    fn append_scope(&self, new_scope: &str) -> Result<(), AnyError>;
}
