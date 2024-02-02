use super::conventional_commit_summary::ConventionalCommitSummary;

#[derive(Debug, Clone)]
pub enum CommitSummary {
    Conventional(ConventionalCommitSummary),
    FreeForm(String),
}
