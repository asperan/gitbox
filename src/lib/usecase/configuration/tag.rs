use crate::{
    domain::semantic_version::SemanticVersion,
    usecase::{
        error::tag_configuration_invariant_error::TagConfigurationInvariantError,
        type_aliases::AnyError,
    },
};

pub struct TagConfiguration {
    version: SemanticVersion,
    message: Option<String>,
    sign: bool,
}

impl TagConfiguration {
    pub fn new(
        version: SemanticVersion,
        message: Option<String>,
        sign: bool,
    ) -> Result<TagConfiguration, AnyError> {
        Self::message_checks(message.as_deref())?;
        Ok(TagConfiguration {
            version,
            message,
            sign,
        })
    }

    pub fn version(&self) -> &SemanticVersion {
        &self.version
    }
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    pub fn sign(&self) -> bool {
        self.sign
    }

    fn message_checks(message: Option<&str>) -> Result<(), TagConfigurationInvariantError> {
        if message.as_ref().is_some_and(|it| it.is_empty()) {
            Err(TagConfigurationInvariantError::new(
                "Message cannot be present but empty",
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::semantic_version::SemanticVersion;

    use super::TagConfiguration;

    #[test]
    fn invariant_correct() {
        let version = SemanticVersion::new(0, 1, 0, None, None);
        let message = Some(String::from("test"));
        let sign = false;
        let result = TagConfiguration::new(version, message, sign);
        assert!(result.is_ok());
    }

    #[test]
    fn invariant_correct_with_no_message() {
        let version = SemanticVersion::new(0, 1, 0, None, None);
        let message = None;
        let sign = false;
        let result = TagConfiguration::new(version, message, sign);
        assert!(result.is_ok());
    }

    #[test]
    fn invariant_wrong() {
        let version = SemanticVersion::new(0, 1, 0, None, None);
        let message = Some(String::new());
        let sign = false;
        let result = TagConfiguration::new(version, message, sign);
        assert!(result.is_err());
    }
}
