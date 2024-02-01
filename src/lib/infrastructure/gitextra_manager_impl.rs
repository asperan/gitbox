use std::{fs::create_dir_all, io::Write, path::Path, rc::Rc};

use crate::{
    application::{
        manager::gitextra_write_manager::GitExtraWriteManager,
        retriever::gitinfo_retriever::GitInfoRetriever,
    },
    usecases::type_aliases::AnyError,
};

const EXTRA_DIR_PATH: &str = "extra";
const TYPES_FILE_PATH: &str = "types.txt";
const SCOPES_FILE_PATH: &str = "scopes.txt";

pub struct GitExtraManagerImpl {
    gitinfo_manager: Rc<dyn GitInfoRetriever>,
}

impl GitExtraManagerImpl {
    pub fn new(gitinfo_manager: Rc<dyn GitInfoRetriever>) -> GitExtraManagerImpl {
        GitExtraManagerImpl { gitinfo_manager }
    }

    fn write_file(&self, path: &Path, content: String) -> Result<(), AnyError> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        writeln!(&mut file, "{}", content.trim())?;
        Ok(())
    }
}

impl GitExtraWriteManager for GitExtraManagerImpl {
    fn update_types(&self, types: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        let content = types.fold(String::new(), |acc, e| acc + "\n" + &e);
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(TYPES_FILE_PATH);
        self.write_file(&path, content)
    }

    fn update_scopes(&self, scopes: Box<dyn Iterator<Item = String>>) -> Result<(), AnyError> {
        let content = scopes.fold(String::new(), |acc, e| acc + "\n" + &e);
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(SCOPES_FILE_PATH);
        self.write_file(&path, content)
    }
}
