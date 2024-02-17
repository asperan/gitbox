use super::conventional_commit_summary::ConventionalCommitSummary;

/**
This enum distinguish conventional commits (i.e. structured messages) and
free-form commits (any other commit).
This enum can be used when the given commits (usually as an input) do not
have a specific type.
See also [ConventionalCommitSummary].
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitSummary {
    Conventional(ConventionalCommitSummary),
    FreeForm(String),
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        commit_summary::CommitSummary, conventional_commit_summary::ConventionalCommitSummary,
    };

    #[test]
    fn debug_commit_summary_conventional() {
        let conv_commit =
            ConventionalCommitSummary::new("feat".to_string(), None, false, "test".to_string());
        let commit_summary = CommitSummary::Conventional(conv_commit.clone());
        assert_eq!(
            format!("{:?}", commit_summary),
            format!("Conventional({:?})", conv_commit)
        );
    }

    #[test]
    fn debug_commit_summary_freeform() {
        let freeform_commit = "a freeform commit".to_string();
        let commit_summary = CommitSummary::FreeForm(freeform_commit.clone());
        assert_eq!(
            format!("{:?}", commit_summary),
            format!("FreeForm({:?})", freeform_commit)
        );
    }

    #[test]
    fn clone_commit_summary_are_equal() {
        let commit =
            ConventionalCommitSummary::new("feat".to_string(), None, false, "test".to_string());
        let commit_summary = CommitSummary::Conventional(commit);
        assert_eq!(commit_summary.clone(), commit_summary);
    }

    #[test]
    fn freeform_and_conventional_are_different() {
        let summary1 = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let summary2 = CommitSummary::FreeForm("freeform string".to_string());
        assert_ne!(summary1, summary2);
    }

    #[test]
    fn conventional_with_same_commit_summary_are_equal() {
        let summary1 = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let summary2 = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        assert_eq!(summary1, summary2);
    }

    #[test]
    fn conventional_with_different_summaries_are_different() {
        let summary1 = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "feat".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        let summary2 = CommitSummary::Conventional(ConventionalCommitSummary::new(
            "fix".to_string(),
            None,
            false,
            "test".to_string(),
        ));
        assert_ne!(summary1, summary2);
    }
}
