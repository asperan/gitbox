use crate::domain::semantic_version::SemanticVersion;

pub struct TagConfiguration {
    version: SemanticVersion,
    message: Option<String>,
    sign: bool,
}

impl TagConfiguration {
    pub fn new(version: SemanticVersion, message: Option<String>, sign: bool) -> TagConfiguration {
        TagConfiguration {
            version,
            message,
            sign,
        }
    }

    pub fn version(&self) -> &SemanticVersion {
        &self.version
    }
    pub fn message(&self) -> &Option<String> {
        &self.message
    }
    pub fn sign(&self) -> bool {
        self.sign
    }
}
