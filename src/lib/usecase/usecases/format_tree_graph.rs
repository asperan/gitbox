use colored::Colorize;

use crate::{
    domain::tree_graph_line::{TreeGraphLine, TreeGraphLineContent},
    usecase::{
        error::format_tree_error::{FormatTreeError, NoCommitsError},
        repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
    },
};

use super::usecase::UseCase;

const TIME_MINIMUM_PADDING: usize = 2;

pub struct FormatTreeGraphUseCase<'a> {
    treegraphline_ingress_repository: &'a dyn TreeGraphLineIngressRepository,
}

impl<'a, 'b: 'a> FormatTreeGraphUseCase<'a> {
    pub fn new(treegraphline_ingress_repository: &'b dyn TreeGraphLineIngressRepository) -> Self {
        FormatTreeGraphUseCase {
            treegraphline_ingress_repository,
        }
    }

    #[inline(always)]
    fn format_line(&self, line: &TreeGraphLine, left_padding: usize) -> Box<str> {
        match line.line_content() {
            TreeGraphLineContent::Metadata(metadata) => format!(
                "{date:>width$} {tree_marks} {hash} {references}",
                date = metadata.relative_date().dimmed(),
                width = left_padding,
                tree_marks = line.tree_marks(),
                hash = metadata.abbreviated_hash().blue(),
                references = metadata.references().yellow(),
            ),
            TreeGraphLineContent::Data(data) => format!(
                "{:>width$} {tree_marks:>1}     {author} {summary}",
                "",
                width = left_padding,
                tree_marks = line.tree_marks(),
                author = data.author().white().bold(),
                summary = data.summary()
            ),
        }
        .into()
    }
}

impl UseCase<Box<str>, FormatTreeError> for FormatTreeGraphUseCase<'_> {
    fn execute(&self) -> Result<Box<str>, FormatTreeError> {
        let lines = self.treegraphline_ingress_repository.graph_lines()?;
        if lines.is_empty() {
            return Err(NoCommitsError::new().into());
        }
        let time_padding = lines
            .iter()
            .filter_map(|it| {
                match it.line_content() {
                    TreeGraphLineContent::Data(_) => None,
                    TreeGraphLineContent::Metadata(metadata) => Some(metadata.relative_date()),
                }
                .map(|it| it.len())
            })
            .max()
            .unwrap_or(0usize);
        let result = lines
            .iter()
            .map(|line| {
                let left_padding = TIME_MINIMUM_PADDING + time_padding;
                self.format_line(line, left_padding)
            })
            .fold(String::new(), |acc, e| acc + "\n" + &e);
        Ok(result.trim_start_matches('\n').into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::tree_graph_line::{
            CommitData, CommitMetadata, TreeGraphLine, TreeGraphLineContent,
        },
        usecase::{
            repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
            type_aliases::AnyError,
            usecases::{format_tree_graph::FormatTreeGraphUseCase, usecase::UseCase},
        },
    };

    struct MockTreeGraphLineIngressRepository {}

    impl TreeGraphLineIngressRepository for MockTreeGraphLineIngressRepository {
        fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError> {
            Ok([
                TreeGraphLine::new(
                    "*",
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
            ]
            .into())
        }
    }

    #[test]
    fn format_header_line() {
        let padding = 16;
        let t = TreeGraphLine::new(
            "*",
            TreeGraphLineContent::Metadata(
                CommitMetadata::new("abcdef0", "( sample date )", "( HEAD -> main )")
                    .expect("Hand-crafted lines are always correct"),
            ),
        );
        let usecase = FormatTreeGraphUseCase::new(&MockTreeGraphLineIngressRepository {});
        let result = usecase.format_line(&t, padding);
        let expected = "\u{1b}[2m ( sample date )\u{1b}[0m * \u{1b}[34mabcdef0\u{1b}[0m \u{1b}[33m( HEAD -> main )\u{1b}[0m";
        assert_eq!(result, expected.into());
    }

    #[test]
    fn format_message_line() {
        let padding = 16;
        let t = TreeGraphLine::new(
            "| ",
            TreeGraphLineContent::Data(
                CommitData::new("asperan:", "test message")
                    .expect("Hand-crafted lines are always correct"),
            ),
        );
        let usecase = FormatTreeGraphUseCase::new(&MockTreeGraphLineIngressRepository {});
        let result = usecase.format_line(&t, padding);
        let expected = "                 |     \u{1b}[1;37masperan:\u{1b}[0m test message";
        assert_eq!(result, expected.into());
    }

    #[test]
    fn execute_complete() {
        let usecase = FormatTreeGraphUseCase::new(&MockTreeGraphLineIngressRepository {});
        let result = usecase
            .execute()
            .expect("The usecase should execute correctly");
        println!("{}", &result);
        let expected = concat!(
            "\u{1b}[2m  ( sample date 1 )\u{1b}[0m * \u{1b}[34mabcdef0\u{1b}[0m \u{1b}[33m( HEAD -> main )\u{1b}[0m\n",
            "                    |     \u{1b}[1;37masperan:\u{1b}[0m test message\n",
            "\u{1b}[2m  ( sample date 2 )\u{1b}[0m * \u{1b}[34m0fedcba\u{1b}[0m \u{1b}[33m\u{1b}[0m\n",
            "                    |     \u{1b}[1;37masperan:\u{1b}[0m another test message",
        );
        assert_eq!(result, expected.into());
    }
}
