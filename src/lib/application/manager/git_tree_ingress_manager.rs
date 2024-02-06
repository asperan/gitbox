use crate::usecases::type_aliases::AnyError;

pub trait GitTreeIngressManager {
    fn commit_tree(&self, format: &str) -> Result<Box<[String]>, AnyError>;
}
