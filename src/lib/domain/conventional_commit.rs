use std::fmt::Display;

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

impl Display for ConventionalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}: {}",
            &self.typ,
            &self
                .scope
                .as_ref()
                .map_or(String::new(), |s| format!("({})", s)),
            if self.breaking { "!" } else { "" },
            &self.summary
        )
    }
}

#[cfg(test)]
mod tests {
    use super::ConventionalCommit;

    #[test]
    fn simple_commit_format() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, false, "test format".to_string());
        assert_eq!(&commit.to_string(), "feat: test format");
    }

    #[test]
    fn scoped_commit_format() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            false,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain): test format");
    }

    #[test]
    fn breaking_commit_format() {
        let commit =
            ConventionalCommit::new("feat".to_string(), None, true, "test format".to_string());
        assert_eq!(&commit.to_string(), "feat!: test format");
    }

    #[test]
    fn breaking_and_scoped_commit_format() {
        let commit = ConventionalCommit::new(
            "feat".to_string(),
            Some("domain".to_string()),
            true,
            "test format".to_string(),
        );
        assert_eq!(&commit.to_string(), "feat(domain)!: test format");
    }
}
