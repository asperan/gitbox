use std::{error::Error, fmt::Display};

/// This error may happen during construction of [TagConfiguration], when
/// one of its invaiants is broken.
#[derive(Debug)]
pub struct TagConfigurationInvariantError {
    message: String,
}

impl TagConfigurationInvariantError {
    pub fn new(message: &str) -> TagConfigurationInvariantError {
        TagConfigurationInvariantError {
            message: message.to_string(),
        }
    }
}

impl Display for TagConfigurationInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to create configuration for tag: {}",
            self.message
        )
    }
}

impl Error for TagConfigurationInvariantError {}
