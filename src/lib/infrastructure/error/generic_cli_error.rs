use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new(message: &str) -> CliError {
        CliError {
            message: message.to_owned(),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
