#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicenseMetadata {
    name: String,
    reference: String,
}

impl LicenseMetadata {
    pub fn new(name: &str, reference: &str) -> Self {
        LicenseMetadata {
            name: name.to_owned(),
            reference: reference.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }
}
