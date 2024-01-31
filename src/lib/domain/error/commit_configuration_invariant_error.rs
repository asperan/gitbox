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

impl Error for CommitConfigurationInvariantError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for CommitConfigurationInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommitConfiguration invariant error: {}", self.message)
    }
}
