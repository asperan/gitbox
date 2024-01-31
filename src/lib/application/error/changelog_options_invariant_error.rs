use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ChangelogOptionsInvariantError {
    message: String,
}

impl ChangelogOptionsInvariantError {
    pub fn new(message: &str) -> ChangelogOptionsInvariantError {
        ChangelogOptionsInvariantError {
            message: message.to_owned(),
        }
    }
}

impl Display for ChangelogOptionsInvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChangelogOptions invariant error: {}", &self.message)
    }
}

impl Error for ChangelogOptionsInvariantError {
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
