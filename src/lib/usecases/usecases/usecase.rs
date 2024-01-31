use crate::usecases::type_aliases::AnyError;

pub trait UseCase<T> {
    fn execute(&self) -> Result<T, AnyError>;
}
