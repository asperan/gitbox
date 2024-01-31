use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct CommandExecutionError {
    full_command: String,
    cause: Box<dyn Error>,
}

impl CommandExecutionError {
    pub fn new(full_command: &str, cause: Box<dyn Error>) -> CommandExecutionError {
        CommandExecutionError {
            full_command: full_command.to_owned(),
            cause,
        }
    }
}

impl Display for CommandExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to run command '{}': {}",
            self.full_command, self.cause
        )
    }
}

impl Error for CommandExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.cause.as_ref())
    }
}
