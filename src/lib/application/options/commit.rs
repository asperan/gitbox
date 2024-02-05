use crate::{application::error::commit_options_invariant_error::CommitOptionsInvariantError, usecases::type_aliases::AnyError};

pub struct CommitOptions {
    commit_type: String,
    scope: Option<String>,
    is_breaking: bool,
    summary: String,
    message: Option<String>,
    quiet: bool,
}

impl CommitOptions {
    pub fn new(
        commit_type: String,
        scope: Option<String>,
        is_breaking: bool,
        summary: String,
        message: Option<String>,
        quiet: bool,
    ) -> Result<CommitOptions, AnyError> {
        Self::check_non_empty(&commit_type, "commit type")?;
        Self::check_non_empty(&summary, "summary")?;
        Self::check_non_empty_if_present(&scope, "scope")?;
        Self::check_non_empty_if_present(&message, "message body")?;
        Ok(CommitOptions {
            commit_type,
            scope,
            is_breaking,
            summary,
            message,
            quiet,
        })
    }

    pub fn commit_type(&self) -> &str {
        &self.commit_type
    }
    pub fn scope(&self) -> &Option<String> {
        &self.scope
    }
    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
    pub fn message(&self) -> &Option<String> {
        &self.message
    }
    pub fn quiet(&self) -> bool {
        self.quiet
    }

    fn check_non_empty(s: &str, what: &str) -> Result<(), CommitOptionsInvariantError> {
        if s.is_empty() {
            Err(CommitOptionsInvariantError::new(what, "must not be empty"))
        } else {
            Ok(())
        }
    }

    fn check_non_empty_if_present(o: &Option<String>, what: &str) -> Result<(), CommitOptionsInvariantError> {
        if o.as_ref().is_some_and(|it| it.is_empty()) {
            Err(CommitOptionsInvariantError::new(what, "must not be empty when present"))
        } else {
            Ok(())
        }
    }
}
