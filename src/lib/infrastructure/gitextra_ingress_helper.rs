use crate::usecases::type_aliases::AnyError;

pub trait GitExtraIngressHelper {
    fn get_types(&self) -> Result<Vec<String>, AnyError>;
    fn get_scopes(&self) -> Result<Vec<String>, AnyError>;
}
