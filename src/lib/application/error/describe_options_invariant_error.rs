use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DescribeOptionsInvariantError {
    message: String,
}

impl DescribeOptionsInvariantError {
    pub fn new(message: &str) -> DescribeOptionsInvariantError {
        DescribeOptionsInvariantError {
            message: message.to_string(),
        }
    }
}

impl Display for DescribeOptionsInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DescribeOptions invariant error: {}", self.message)
    }
}

impl Error for DescribeOptionsInvariantError {}
