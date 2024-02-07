use crate::usecase::type_aliases::AnyError;

pub trait GitInfoIngressManager {
    fn git_dir(&self) -> Result<String, AnyError>;
}
