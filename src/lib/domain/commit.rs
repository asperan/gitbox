use super::conventional_commit_summary::ConventionalCommitSummary;

#[derive(Debug)]
pub enum CommitSummary {
    Conventional(ConventionalCommitSummary),
    FreeForm(String),
}
