use crate::usecases::type_aliases::AnyError;

pub trait VersionRetriever {
    fn last_version(&self) -> Result<Option<String>, AnyError>;
    fn last_stable_version(&self) -> Result<Option<String>, AnyError>;
}
