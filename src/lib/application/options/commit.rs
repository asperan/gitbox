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
    ) -> CommitOptions {
        CommitOptions {
            commit_type,
            scope,
            is_breaking,
            summary,
            message,
            quiet,
        }
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
}
