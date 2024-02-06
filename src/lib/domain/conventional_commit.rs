use super::conventional_commit_summary::ConventionalCommitSummary;

pub struct ConventionalCommit {
    summary: ConventionalCommitSummary,
    message: Option<String>,
}

impl ConventionalCommit {
    pub fn new(
        typ: String,
        scope: Option<String>,
        breaking: bool,
        summary: String,
        message: Option<String>,
    ) -> ConventionalCommit {
        ConventionalCommit {
            summary: ConventionalCommitSummary::new(typ, scope, breaking, summary),
            message,
        }
    }

    pub fn summary(&self) -> &ConventionalCommitSummary {
        &self.summary
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }
}
