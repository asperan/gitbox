use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CommitOptionsInvariantError {
    what: String,
    message: String,
}

impl CommitOptionsInvariantError {
    pub fn new(what: &str, message: &str) -> CommitOptionsInvariantError {
        CommitOptionsInvariantError {
            what: what.to_string(),
            message: message.to_string(),
        }
    }
}

impl Display for CommitOptionsInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while constructing commit options: {} {}",
            self.what, self.message
        )
    }
}

impl Error for CommitOptionsInvariantError {}
