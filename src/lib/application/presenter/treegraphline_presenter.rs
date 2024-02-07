use std::str::FromStr;

use crate::{
    application::error::treegraphline_format_error::TreeGraphLineFormatError,
    usecase::{tree_graph_line::TreeGraphLine, type_aliases::AnyError},
};

impl FromStr for TreeGraphLine {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // if the separator are 3, the resulting array will have 4 values:
        // "1sep2sep3sep4" => [1, 2, 3, 4]
        if s.matches(TreeGraphLine::separator()).count() == 3 {
            let raw_array: Box<[&str]> = s.split(TreeGraphLine::separator()).collect();
            Ok(TreeGraphLine::new(
                raw_array[1].to_owned(),
                raw_array[0].to_owned(),
                raw_array[2].to_owned(),
                raw_array[3].to_owned(),
            ))
        } else {
            Err(Box::new(TreeGraphLineFormatError::new(&format!(
                "the line must contain TreeGraphLine separator '{}' 3 times",
                TreeGraphLine::separator()
            ))))
        }
    }
}
