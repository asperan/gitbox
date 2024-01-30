use crate::domain::{configuration::commit::CommitConfiguration, type_aliases::AnyError};

use super::usecase::UseCase;

#[derive(Debug)]
pub struct CreateCommitMessageUseCase {
    configuration: CommitConfiguration,
}

impl CreateCommitMessageUseCase {
    pub fn new(configuration: CommitConfiguration) -> CreateCommitMessageUseCase {
        CreateCommitMessageUseCase { configuration }
    }

    fn format_scope(&self) -> String {
        match self.configuration.scope() {
            Some(s) => format!("({})", s),
            None => String::new(),
        }
    }

    fn format_breaking(&self) -> String {
        if self.configuration.is_breaking() {
            String::from("!")
        } else {
            String::new()
        }
    }

    fn format_message(&self) -> String {
        match self.configuration.message() {
            Some(m) => format!("\n\n{}", m),
            None => String::new(),
        }
    }
}

impl UseCase<String> for CreateCommitMessageUseCase {
    fn execute(&self) -> Result<String, AnyError> {
        Ok(format!(
            "{}{}{}: {}{}",
            self.configuration.typ(),
            self.format_scope(),
            self.format_breaking(),
            self.configuration.summary(),
            self.format_message()
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::configuration::commit::CommitConfiguration,
        usecases::usecases::{create_commit_message::CreateCommitMessageUseCase, usecase::UseCase},
    };

    fn simple_configuration() -> CommitConfiguration {
        CommitConfiguration::new("feat".to_string(), None, false, "test".to_string(), None)
            .expect("This configuration is well-formed")
    }

    fn full_configuration() -> CommitConfiguration {
        CommitConfiguration::new(
            "feat".to_string(),
            Some("test".to_string()),
            true,
            "test".to_string(),
            Some("Message body".to_string()),
        )
        .expect("This configuration is well-formed")
    }

    #[test]
    fn format_scope_empty() {
        let config = simple_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_scope(), String::new());
    }

    #[test]
    fn format_scope_present() {
        let config = full_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_scope(), String::from("(test)"));
    }

    #[test]
    fn format_is_breaking() {
        let config = full_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_breaking(), String::from("!"));
    }

    #[test]
    fn format_is_not_breaking() {
        let config = simple_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_breaking(), String::new());
    }

    #[test]
    fn format_message_empty() {
        let config = simple_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_message(), String::new());
    }

    #[test]
    fn format_message_present() {
        let config = full_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        assert_eq!(usecase.format_message(), String::from("\n\nMessage body"));
    }

    #[test]
    fn execute_usecase_simplest() {
        let config = simple_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        let result = usecase.execute();
        let expected = "feat: test";
        assert!(match result {
            Ok(message) => message == expected,
            _ => false,
        });
    }

    #[test]
    fn execute_usecase_fullest() {
        let config = full_configuration();
        let usecase = CreateCommitMessageUseCase::new(config);
        let result = usecase.execute();
        let expected = "feat(test)!: test\n\nMessage body";
        assert!(match result {
            Ok(message) => message == expected,
            _ => false,
        });
    }
}
