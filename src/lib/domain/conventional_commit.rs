#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionalCommit {
    typ: String,
    scope: Option<String>,
    breaking: bool,
    summary: String,
}

impl ConventionalCommit {
    pub fn new(
        typ: String,
        scope: Option<String>,
        breaking: bool,
        summary: String,
    ) -> ConventionalCommit {
        ConventionalCommit {
            typ,
            scope,
            breaking,
            summary,
        }
    }

    pub fn typ(&self) -> &String {
        &self.typ
    }

    pub fn scope(&self) -> &Option<String> {
        &self.scope
    }

    pub fn breaking(&self) -> bool {
        self.breaking
    }

    pub fn summary(&self) -> &String {
        &self.summary
    }
}

