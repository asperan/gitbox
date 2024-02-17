/*
Summary of a conventional commit.

It contains the type, the optional scope, the breaking-ness
and the first line of the commit message.

For more information about the specification of a conventional commit,
see [the official documentation](https://www.conventionalcommits.org/en/v1.0.0/).

If the complete message is needed, see [ConventionalCommit].
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionalCommitSummary {
    typ: String,
    scope: Option<String>,
    breaking: bool,
    summary: String,
}

impl ConventionalCommitSummary {
    pub fn new(
        typ: String,
        scope: Option<String>,
        breaking: bool,
        summary: String,
    ) -> ConventionalCommitSummary {
        ConventionalCommitSummary {
            typ,
            scope,
            breaking,
            summary,
        }
    }

    pub fn typ(&self) -> &str {
        &self.typ
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }

    pub fn breaking(&self) -> bool {
        self.breaking
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }
}
