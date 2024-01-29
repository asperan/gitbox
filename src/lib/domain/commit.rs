use super::conventional_commit::ConventionalCommit;

#[derive(Debug)]
pub enum Commit {
    Conventional(ConventionalCommit),
    FreeForm(String),
}
