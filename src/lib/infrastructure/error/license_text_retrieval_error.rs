use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LicenseTextRetrievalError {
    message: String,
}

impl LicenseTextRetrievalError {
    pub fn new(message: &str) -> Self {
        LicenseTextRetrievalError {
            message: message.to_string(),
        }
    }
}

impl Display for LicenseTextRetrievalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to retreive license text: {}", self.message)
    }
}

impl Error for LicenseTextRetrievalError {}
