use regex::Regex;

use crate::usecase::{
    repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
    tree_graph_line::TreeGraphLine, type_aliases::AnyError,
};

use super::usecase::UseCase;

const TIME_MINIMUM_PADDING: usize = 2;

pub struct FormatTreeGraphUseCase {
    treegraphline_ingress_repository: Box<dyn TreeGraphLineIngressRepository>,
}

impl FormatTreeGraphUseCase {
    pub fn new(treegraphline_ingress_repository: Box<dyn TreeGraphLineIngressRepository>) -> Self {
        FormatTreeGraphUseCase {
            treegraphline_ingress_repository,
        }
    }

    #[inline(always)]
    fn format_line(&self, line: &TreeGraphLine, left_padding: usize) -> Box<str> {
        format!(
            "{date:>width$} {tree_mark} {pointers} {commit_text}",
            date = line.date(),
            width = left_padding,
            tree_mark = line.tree_mark(),
            pointers = line.pointers(),
            commit_text = line.message(),
        )
        .into()
    }
}

impl UseCase<Box<str>> for FormatTreeGraphUseCase {
    fn execute(&self) -> Result<Box<str>, AnyError> {
        let lines = self.treegraphline_ingress_repository.graph_lines()?;
        let time_color_length = {
            let time_regex = Regex::new("\\([a-z0-9 ,]+\\)").unwrap();
            let first_line_time = &lines[0].date();
            match time_regex.captures(first_line_time) {
                Some(captures) => {
                    first_line_time.len()
                        - captures.get(0).expect("the groups should be there").len()
                }
                None => 0,
            }
        };
        let time_padding = lines
            .iter()
            .filter_map(|it| {
                if !it.date().is_empty() {
                    Some(it.date().len() - time_color_length)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0usize);
        let result = lines
            .iter()
            .map(|line| {
                let left_padding = TIME_MINIMUM_PADDING
                    + time_padding
                    + if !line.date().is_empty() {
                        time_color_length
                    } else {
                        0
                    };
                self.format_line(line, left_padding)
            })
            .fold(String::new(), |acc, e| acc + "\n" + &e);
        Ok(result.trim_start_matches('\n').into())
    }
}

#[cfg(test)]
mod tests {
    use crate::usecase::{
        repository::treegraphline_ingress_repository::TreeGraphLineIngressRepository,
        tree_graph_line::TreeGraphLine,
        type_aliases::AnyError,
        usecases::{format_tree_graph::FormatTreeGraphUseCase, usecase::UseCase},
    };

    struct MockTreeGraphLineIngressRepository {}

    impl TreeGraphLineIngressRepository for MockTreeGraphLineIngressRepository {
        fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError> {
            Ok([
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
            ]
            .into())
        }
    }

    #[test]
    fn format_header_line() {
        let padding = 16;
        let t = TreeGraphLine::new(
            "( sample date )".to_owned(),
            "* abcdef".to_owned(),
            "( HEAD -> main )".to_owned(),
            String::new(),
        );
        let usecase = FormatTreeGraphUseCase::new(Box::new(MockTreeGraphLineIngressRepository {}));
        let result = usecase.format_line(&t, padding);
        let expected = " ( sample date ) * abcdef ( HEAD -> main ) ";
        assert_eq!(result, expected.into());
    }

    #[test]
    fn format_message_line() {
        let padding = 16;
        let t = TreeGraphLine::new(
            String::new(),
            "| ".to_owned(),
            "".to_owned(),
            "asperan: test message".to_owned(),
        );
        let usecase = FormatTreeGraphUseCase::new(Box::new(MockTreeGraphLineIngressRepository {}));
        let result = usecase.format_line(&t, padding);
        let expected = "                 |   asperan: test message";
        assert_eq!(result, expected.into());
    }

    #[test]
    fn execute_complete() {
        let usecase = FormatTreeGraphUseCase::new(Box::new(MockTreeGraphLineIngressRepository {}));
        let result = usecase
            .execute()
            .expect("The usecase should execute correctly");
        println!("{}", &result);
        let expected = concat!(
            "  ( sample date 1 ) * abcdef ( HEAD -> main ) \n",
            "                    |   asperan: test message\n",
            "  ( sample date 2 ) * fedcba  \n",
            "                    |   asperan: another test message",
        );
        assert_eq!(result, expected.into());
    }
}
