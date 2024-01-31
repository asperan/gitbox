use crate::domain::type_aliases::AnyError;

pub trait GitInfoRetriever {
    fn git_dir(&self) -> Result<String, AnyError>;
}
