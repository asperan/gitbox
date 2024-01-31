use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CommitConfigurationInvariantError {
    message: String,
}

impl CommitConfigurationInvariantError {
    pub fn new(message: &str) -> CommitConfigurationInvariantError {
        CommitConfigurationInvariantError {
            message: message.to_string(),
        }
    }
}

impl Error for CommitConfigurationInvariantError {}

impl Display for CommitConfigurationInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommitConfiguration invariant error: {}", self.message)
    }
}
