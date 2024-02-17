use crate::usecase::{
    error::commit_configuration_invariant_error::CommitConfigurationInvariantError,
    type_aliases::AnyError,
};

#[derive(Debug)]
pub struct CommitConfiguration {
    commit_type: String,
    scope: Option<String>,
    is_breaking: bool,
    summary: String,
    message: Option<String>,
}

impl CommitConfiguration {
    pub fn new(
        commit_type: String,
        scope: Option<String>,
        is_breaking: bool,
        summary: String,
        message: Option<String>,
    ) -> Result<CommitConfiguration, AnyError> {
        Self::type_checks(&commit_type)?;
        Self::scope_checks(scope.as_deref())?;
        Self::summary_checks(&summary)?;
        Self::message_checks(message.as_deref())?;
        Ok(CommitConfiguration {
            commit_type,
            scope,
            is_breaking,
            summary,
            message,
        })
    }

    pub fn typ(&self) -> &str {
        &self.commit_type
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    fn type_checks(t: &str) -> Result<(), CommitConfigurationInvariantError> {
        if t.is_empty() {
            return Err(CommitConfigurationInvariantError::new(
                "Type length cannot be 0",
            ));
        }
        Ok(())
    }

    fn scope_checks(s: Option<&str>) -> Result<(), CommitConfigurationInvariantError> {
        if s.as_ref().is_some_and(|it| it.is_empty()) {
            return Err(CommitConfigurationInvariantError::new(
                "Scope length cannot be 0 if present",
            ));
        }
        Ok(())
    }

    fn summary_checks(s: &str) -> Result<(), CommitConfigurationInvariantError> {
        if s.is_empty() {
            return Err(CommitConfigurationInvariantError::new(
                "Summary length cannot be 0",
            ));
        }
        Ok(())
    }

    fn message_checks(m: Option<&str>) -> Result<(), CommitConfigurationInvariantError> {
        if m.as_ref().is_some_and(|it| it.is_empty()) {
            return Err(CommitConfigurationInvariantError::new(
                "Message length cannot be 0 if present",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::CommitConfiguration;

    #[test]
    fn type_invariants_ok() {
        let t = String::from("type");
        let result = CommitConfiguration::type_checks(&t);
        assert!(result.is_ok());
    }

    #[test]
    fn type_invariants_wrong() {
        let t = String::new();
        let result = CommitConfiguration::type_checks(&t);
        assert!(result.is_err());
    }

    #[test]
    fn scope_invariants_ok() {
        let s1 = Some(String::from("api"));
        let s2: Option<&str> = None;
        let result1 = CommitConfiguration::scope_checks(s1.as_deref());
        let result2 = CommitConfiguration::scope_checks(s2.as_deref());
        assert!(result1.is_ok() && result2.is_ok());
    }

    #[test]
    fn scope_invariants_wrong() {
        let s = Some(String::new());
        let result = CommitConfiguration::scope_checks(s.as_deref());
        assert!(result.is_err());
    }

    #[test]
    fn summary_invariants_ok() {
        let s = String::from("add test");
        let result = CommitConfiguration::summary_checks(&s);
        assert!(result.is_ok());
    }

    #[test]
    fn summary_invariants_wrong() {
        let s = String::new();
        let result = CommitConfiguration::summary_checks(&s);
        assert!(result.is_err());
    }

    #[test]
    fn message_invariants_ok() {
        let m1 = Some(String::from("Message body"));
        let m2: Option<&str> = None;
        let result1 = CommitConfiguration::message_checks(m1.as_deref());
        let result2 = CommitConfiguration::message_checks(m2.as_deref());
        assert!(result1.is_ok() && result2.is_ok());
    }

    #[test]
    fn message_invariants_wrong() {
        let m = Some(String::new());
        let result = CommitConfiguration::message_checks(m.as_deref());
        assert!(result.is_err());
    }
}
