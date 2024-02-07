use std::{rc::Rc, str::FromStr};

use crate::{
    application::manager::git_tree_ingress_manager::GitTreeIngressManager,
    usecase::{
        repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
        tree_graph_line::TreeGraphLine, type_aliases::AnyError,
    },
};

pub struct TreeGraphLineIngressRepositoryImpl {
    treegraphline_ingress_manager: Rc<dyn GitTreeIngressManager>,
}

impl TreeGraphLineIngressRepositoryImpl {
    pub fn new(treegraphline_ingress_manager: Rc<dyn GitTreeIngressManager>) -> Self {
        TreeGraphLineIngressRepositoryImpl {
            treegraphline_ingress_manager,
        }
    }
}

impl TreeGraphLineIngressRepository for TreeGraphLineIngressRepositoryImpl {
    fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError> {
        let lines = self
            .treegraphline_ingress_manager
            .commit_tree(TreeGraphLine::format())?;
        lines
            .iter()
            .map(|it| TreeGraphLine::from_str(it))
            .collect::<Result<Box<[TreeGraphLine]>, AnyError>>()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        application::{
            manager::git_tree_ingress_manager::GitTreeIngressManager,
            repository_impl::treegraphline_ingress_repository_impl::TreeGraphLineIngressRepositoryImpl,
        },
        usecase::{
            repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
            tree_graph_line::TreeGraphLine, type_aliases::AnyError,
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
                Ok([format!("* abcdef{separator}( sample date 1 ){separator}( HEAD -> main ){separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}{separator}{separator}asperan: test message", separator = TreeGraphLine::separator()),
                format!("* fedcba{separator}( sample date 2 ){separator}{separator}", separator = TreeGraphLine::separator()),
                format!("| {separator}{separator}{separator}asperan: another test message", separator = TreeGraphLine::separator())].into())
            }
        }
    }

    #[test]
    fn correct_deserialization_of_tree_lines() {
        let git_tree_ingress_manager = Rc::new(MockGitTreeIngressManager {
            produce_bad_lines: false,
        });
        let repository_impl = TreeGraphLineIngressRepositoryImpl::new(git_tree_ingress_manager);
        let result = repository_impl.graph_lines();
        assert!(result.is_ok());
        let expected = [
            TreeGraphLine::new(
                "( sample date 1 )".to_owned(),
                "* abcdef".to_owned(),
                "( HEAD -> main )".to_owned(),
                String::new(),
            ),
            TreeGraphLine::new(
                String::new(),
                "| ".to_owned(),
                String::new(),
                "asperan: test message".to_owned(),
            ),
            TreeGraphLine::new(
                "( sample date 2 )".to_owned(),
                "* fedcba".to_owned(),
                String::new(),
                String::new(),
            ),
            TreeGraphLine::new(
                String::new(),
                "| ".to_owned(),
                String::new(),
                "asperan: another test message".to_owned(),
            ),
        ];
        assert_eq!(result.expect("Just asserted the Ok-ness"), expected.into())
    }

    #[test]
    fn wrong_lines() {
        let git_tree_ingress_manager = Rc::new(MockGitTreeIngressManager {
            produce_bad_lines: true,
        });
        let repository_impl = TreeGraphLineIngressRepositoryImpl::new(git_tree_ingress_manager);
        let result = repository_impl.graph_lines();
        assert!(result.is_err());
    }
}
