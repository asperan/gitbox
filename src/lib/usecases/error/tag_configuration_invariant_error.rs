use std::{error::Error, fmt::Display};

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
