use std::{
    fs::{create_dir_all, read_to_string},
    io::Write,
    path::Path,
    rc::Rc,
};

use crate::domain::constant::DEFAULT_COMMIT_TYPES;
use crate::{
    application::{
        manager::gitextra_egress_manager::GitExtraEgressManager,
        manager::gitinfo_ingress_manager::GitInfoIngressManager,
    },
    usecases::type_aliases::AnyError,
};

use super::{
    gitextra_egress_helper::GitExtraEgressHelper, gitextra_ingress_helper::GitExtraIngressHelper,
};

const EXTRA_DIR_PATH: &str = "extra";
const TYPES_FILE_PATH: &str = "types.txt";
const SCOPES_FILE_PATH: &str = "scopes.txt";

pub struct GitExtraManagerImpl {
    gitinfo_manager: Rc<dyn GitInfoIngressManager>,
}

impl GitExtraManagerImpl {
    pub fn new(gitinfo_manager: Rc<dyn GitInfoIngressManager>) -> GitExtraManagerImpl {
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

    fn append_to_file(&self, path: &Path, content: &str) -> Result<(), AnyError> {
        let mut f = std::fs::File::options().append(true).open(path).unwrap();
        write!(f, "\n{}", content)?;
        Ok(())
    }
}

impl GitExtraEgressManager for GitExtraManagerImpl {
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

impl GitExtraIngressHelper for GitExtraManagerImpl {
    fn get_types(&self) -> Result<Vec<String>, AnyError> {
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(TYPES_FILE_PATH);
        Ok(read_to_string(path)?
            .split('\n')
            .filter(|it| !it.is_empty() && !DEFAULT_COMMIT_TYPES.contains(it))
            .chain(DEFAULT_COMMIT_TYPES.into_iter())
            .map(|it| it.to_string())
            .collect())
    }

    fn get_scopes(&self) -> Result<Vec<String>, AnyError> {
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(SCOPES_FILE_PATH);
        Ok(read_to_string(path)?
            .split('\n')
            .filter(|it| !it.is_empty())
            .map(|it| it.to_string())
            .collect())
    }
}

impl GitExtraEgressHelper for GitExtraManagerImpl {
    fn append_type(&self, new_type: &str) -> Result<(), AnyError> {
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(TYPES_FILE_PATH);
        self.append_to_file(&path, new_type)
    }

    fn append_scope(&self, new_scope: &str) -> Result<(), AnyError> {
        let path = Path::new(&self.gitinfo_manager.git_dir()?)
            .join(EXTRA_DIR_PATH)
            .join(SCOPES_FILE_PATH);
        self.append_to_file(&path, new_scope)
    }
}
