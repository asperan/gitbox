use std::{fs::create_dir_all, io::Write, path::Path};

use crate::{
    application::manager::license_text_egress_manager::LicenseTextEgressManager,
    usecases::type_aliases::AnyError,
};

pub struct FileWriter {}

impl FileWriter {
    pub fn new() -> Self {
        FileWriter {}
    }
}

impl LicenseTextEgressManager for FileWriter {
    fn write_license(&self, filepath: &str, text: &str) -> Result<(), AnyError> {
        let path = Path::new(filepath);
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        writeln!(&mut file, "{}", text.trim())?;
        Ok(())
    }
}
