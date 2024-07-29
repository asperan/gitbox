use crate::{domain::tree_graph_line::TreeGraphLine, usecase::type_aliases::AnyError};

pub trait TreeGraphLineIngressRepository {
    fn graph_lines(&self) -> Result<Box<[TreeGraphLine]>, AnyError>;
}
