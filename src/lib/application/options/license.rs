pub struct LicenseOptions {
    path: Box<str>,
}

impl LicenseOptions {
    pub fn new(path: &str) -> Self {
        LicenseOptions { path: path.into() }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
