use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TreeGraphLineFormatError {
    message: String,
}

impl TreeGraphLineFormatError {
    pub fn new(message: &str) -> Self {
        TreeGraphLineFormatError { message: message.to_owned() }
    }
}

impl Display for TreeGraphLineFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse tree graph line: {}", self.message)
    }
}

impl Error for TreeGraphLineFormatError {}
