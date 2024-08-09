use std::str::FromStr;

use crate::{
    application::error::treegraphline_format_error::{
        LineInvariantError, SeparatorNumberError, TreeGraphLineParseError,
    },
    domain::tree_graph_line::{CommitData, CommitMetadata, TreeGraphLine, TreeGraphLineContent},
};

impl FromStr for TreeGraphLine {
    type Err = TreeGraphLineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let correct_number_of_separators = TreeGraphLine::format()
            .matches(TreeGraphLine::separator())
            .count()
            / 2;
        let actual_number_of_separators = s.matches(TreeGraphLine::separator()).count();
        if actual_number_of_separators == correct_number_of_separators {
            let raw_array: Box<[&str]> = s.split(TreeGraphLine::separator()).collect();
            if (!raw_array[TreeGraphLine::abbreviated_hash_position()].is_empty()
                || !raw_array[TreeGraphLine::relative_date_position()].is_empty()
                || !raw_array[TreeGraphLine::references_position()].is_empty())
                && (!raw_array[TreeGraphLine::author_position()].is_empty()
                    || !raw_array[TreeGraphLine::summary_position()].is_empty())
            {
                Err(LineInvariantError {}.into())
            } else {
                let line_content =
                    if raw_array[TreeGraphLine::abbreviated_hash_position()].is_empty() {
                        TreeGraphLineContent::Data(CommitData::new(
                            raw_array[TreeGraphLine::author_position()],
                            raw_array[TreeGraphLine::summary_position()],
                        )?)
                    } else {
                        TreeGraphLineContent::Metadata(CommitMetadata::new(
                            raw_array[TreeGraphLine::abbreviated_hash_position()],
                            raw_array[TreeGraphLine::relative_date_position()],
                            raw_array[TreeGraphLine::references_position()],
                        )?)
                    };
                Ok(TreeGraphLine::new(
                    raw_array[TreeGraphLine::tree_marks_position()],
                    line_content,
                ))
            }
        } else {
            Err(SeparatorNumberError::new(
                correct_number_of_separators,
                actual_number_of_separators,
            )
            .into())
        }
    }
}
