use std::str::FromStr;

use crate::{
    application::{
        error::treegraphline_format_error::TreeGraphLineParseError,
        manager::git_tree_ingress_manager::GitTreeIngressManager,
    },
    domain::tree_graph_line::TreeGraphLine,
    usecase::{
        repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct TreeGraphLineIngressRepositoryImpl<'a> {
    treegraphline_ingress_manager: &'a dyn GitTreeIngressManager,
}

impl<'a, 'b: 'a> TreeGraphLineIngressRepositoryImpl<'a> {
    pub fn new(treegraphline_ingress_manager: &'b dyn GitTreeIngressManager) -> Self {
        TreeGraphLineIngressRepositoryImpl {
            treegraphline_ingress_manager,
        }
    }
}

impl TreeGraphLineIngressRepository for TreeGraphLineIngressRepositoryImpl<'_> {
    fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError> {
        let lines = self
            .treegraphline_ingress_manager
            .commit_tree(TreeGraphLine::format())?;
        Ok(lines
            .iter()
            .map(|it| TreeGraphLine::from_str(it))
            .collect::<Result<Box<[TreeGraphLine]>, TreeGraphLineParseError>>()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            manager::git_tree_ingress_manager::GitTreeIngressManager,
            repository_impl::treegraphline_ingress_repository_impl::TreeGraphLineIngressRepositoryImpl,
        },
        domain::tree_graph_line::{
            CommitData, CommitMetadata, TreeGraphLine, TreeGraphLineContent,
        },
        usecase::{
            repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
            type_aliases::AnyError,
        },
    };

    struct MockGitTreeIngressManager {
        produce_bad_lines: bool,
    }

    impl GitTreeIngressManager for MockGitTreeIngressManager {
        fn commit_tree(&self, _format: &str) -> Result<Box<[String]>, AnyError> {
            if self.produce_bad_lines {
                Ok([format!("* abcdef{separator}( sample date 1 ){separator}( HEAD -> main ){separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}THIS LINE IS BAD{separator}asperan: test message", separator = TreeGraphLine::separator()),
                format!("* fedcba{separator}( sample date 2 ){separator}{separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}{separator}{separator}asperan: another test message", separator = TreeGraphLine::separator())].into())
            } else {
                Ok([format!("* {separator}abcdef0{separator}( sample date 1 ){separator}( HEAD -> main ){separator}{separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}{separator}{separator}{separator}asperan: {separator}test message", separator = TreeGraphLine::separator()),
                format!("* {separator}0fedcba{separator}( sample date 2 ){separator}{separator}{separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}{separator}{separator}{separator}asperan: {separator}another test message", separator = TreeGraphLine::separator())].into())
            }
        }
    }

    #[test]
    fn correct_deserialization_of_tree_lines() {
        let git_tree_ingress_manager = MockGitTreeIngressManager {
            produce_bad_lines: false,
        };
        let repository_impl = TreeGraphLineIngressRepositoryImpl::new(&git_tree_ingress_manager);
        let result = repository_impl.graph_lines();
        assert!(result.is_ok());
        let expected = [
            TreeGraphLine::new(
                "* ",
                TreeGraphLineContent::Metadata(
                    CommitMetadata::new("abcdef0", "( sample date 1 )", "( HEAD -> main )")
                        .expect("Hand-crafted lines are always correct"),
                ),
            ),
            TreeGraphLine::new(
                "| ",
                TreeGraphLineContent::Data(
                    CommitData::new("asperan:", "test message")
                        .expect("Hand-crafted lines are always correct"),
                ),
            ),
            TreeGraphLine::new(
                "*",
                TreeGraphLineContent::Metadata(
                    CommitMetadata::new("0fedcba", "( sample date 2 )", "")
                        .expect("Hand-crafted lines are always correct"),
                ),
            ),
            TreeGraphLine::new(
                "| ",
                TreeGraphLineContent::Data(
                    CommitData::new("asperan:", "another test message")
                        .expect("Hand-crafted lines are always correct"),
                ),
            ),
        ];
        assert_eq!(result.expect("Just asserted the Ok-ness"), expected.into())
    }

    #[test]
    fn wrong_lines() {
        let git_tree_ingress_manager = MockGitTreeIngressManager {
            produce_bad_lines: true,
        };
        let repository_impl = TreeGraphLineIngressRepositoryImpl::new(&git_tree_ingress_manager);
        let result = repository_impl.graph_lines();
        assert!(result.is_err());
    }
}
