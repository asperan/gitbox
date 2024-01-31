use std::{path::Path, rc::Rc};

use crate::{domain::type_aliases::AnyError, usecases::repository::gitinfo_repository::GitInfoRepository};

use super::retriever::gitinfo_retriever::GitInfoRetriever;

pub struct GitInfoRepositoryImpl {
    gitinfo_retriever: Rc<dyn GitInfoRetriever>,
}

impl GitInfoRepositoryImpl {
    pub fn new(gitinfo_retriever: Rc<dyn GitInfoRetriever>) -> GitInfoRepositoryImpl {
        GitInfoRepositoryImpl { gitinfo_retriever }
    }
}

impl GitInfoRepository for GitInfoRepositoryImpl {
    fn git_dir(&self) -> Result<Box<Path>, AnyError> {
        self.gitinfo_retriever
            .git_dir()
            .map(|it| Box::from(Path::new(&it)))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{application::retriever::gitinfo_retriever::GitInfoRetriever, usecases::repository::gitinfo_repository::GitInfoRepository};

    use super::GitInfoRepositoryImpl;

    struct MockGitInfoRetriever {}
    impl GitInfoRetriever for MockGitInfoRetriever {
        fn git_dir(&self) -> Result<String, crate::domain::type_aliases::AnyError> {
            Ok(String::from("/absolute/path/to/a/git/dir"))
        }
    }

    #[test]
    fn git_dir_basic() {
        let repository = GitInfoRepositoryImpl::new(Rc::new(MockGitInfoRetriever {}));
        let git_dir = repository.git_dir();
        assert!(git_dir.is_ok());
        assert_eq!(git_dir.expect("Just asserted its OK-ness").to_str().expect("The inner path contains only ASCII characters"), "/absolute/path/to/a/git/dir");
    }
}
