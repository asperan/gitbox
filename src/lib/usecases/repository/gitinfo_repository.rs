use std::path::Path;

use crate::domain::type_aliases::AnyError;

pub trait GitInfoRepository {
    fn git_dir(&self) -> Result<Box<Path>, AnyError>;
}
