use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CommitSummaryParsingError {
    message: String,
}

impl CommitSummaryParsingError {
    pub fn new(message: &str) -> CommitSummaryParsingError {
        CommitSummaryParsingError {
            message: message.to_owned(),
        }
    }
}

impl Display for CommitSummaryParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse commit summary: {}", self.message)
    }
}

impl Error for CommitSummaryParsingError {}
