use super::conventional_commit_summary::ConventionalCommitSummary;

/**
This enum distinguish conventional commits (i.e. structured messages) and
free-form commits (any other commit).
This enum can be used when the given commits (usually as an input) do not
have a specific type.
See also [ConventionalCommitSummary].
*/
#[derive(Debug, Clone)]
pub enum CommitSummary {
    Conventional(ConventionalCommitSummary),
    FreeForm(String),
}
