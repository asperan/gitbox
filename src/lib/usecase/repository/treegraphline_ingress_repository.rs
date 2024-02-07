use crate::usecase::{tree_graph_line::TreeGraphLine, type_aliases::AnyError};

pub trait TreeGraphLineIngressRepository {
    fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError>;
}
