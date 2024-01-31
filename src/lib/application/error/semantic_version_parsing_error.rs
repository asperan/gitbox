use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct SemanticVersionParsingError {
    wrong_version: String,
}

impl SemanticVersionParsingError {
    pub fn new(wrong_version: &str) -> SemanticVersionParsingError {
        SemanticVersionParsingError {
            wrong_version: wrong_version.to_string(),
        }
    }

    pub fn wrong_version(&self) -> &str {
        &self.wrong_version
    }
}

impl Display for SemanticVersionParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version '{}' is not semantic", self.wrong_version)
    }
}

impl Error for SemanticVersionParsingError {
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
