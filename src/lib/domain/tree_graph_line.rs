use super::error::tree_graph_line_invariant_error::{
    AuthorInvariantError, DataInvariantError, DateInvariantError, HashInvariantError,
    MetadataInvariantError, SummaryInvariantError,
};

const TREE_FORMAT: &str = "§%h§(%cr)§%d§§%n§§§§%an: §%s";
const FIELD_SEPARATOR: char = '§';
const HASH_EXPECTED_LENGTH: usize = 7;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeGraphLine {
    tree_marks: String,
    line_content: TreeGraphLineContent,
}

impl TreeGraphLine {
    pub const fn format() -> &'static str {
        TREE_FORMAT
    }

    pub const fn tree_marks_position() -> usize {
        0
    }

    pub const fn abbreviated_hash_position() -> usize {
        1
    }

    pub const fn relative_date_position() -> usize {
        2
    }

    pub const fn references_position() -> usize {
        3
    }

    pub const fn author_position() -> usize {
        4
    }

    pub const fn summary_position() -> usize {
        5
    }

    pub const fn separator() -> char {
        FIELD_SEPARATOR
    }

    pub fn new(tree_marks: &str, line_content: TreeGraphLineContent) -> Self {
        Self {
            tree_marks: tree_marks.trim().into(),
            line_content,
        }
    }

    pub fn tree_marks(&self) -> &str {
        &self.tree_marks
    }

    pub fn line_content(&self) -> &TreeGraphLineContent {
        &self.line_content
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TreeGraphLineContent {
    Metadata(CommitMetadata),
    Data(CommitData),
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommitMetadata {
    abbreviated_hash: String,
    relative_date: String,
    references: String,
}

impl CommitMetadata {
    pub fn new(
        abbreviated_hash: &str,
        relative_date: &str,
        references: &str,
    ) -> Result<Self, MetadataInvariantError> {
        Ok(Self {
            abbreviated_hash: Self::check_hash(abbreviated_hash.trim())?.into(),
            relative_date: Self::check_date(relative_date.trim())?.into(),
            references: references.trim().into(),
        })
    }

    pub fn abbreviated_hash(&self) -> &str {
        &self.abbreviated_hash
    }

    pub fn relative_date(&self) -> &str {
        &self.relative_date
    }

    pub fn references(&self) -> &str {
        &self.references
    }

    fn check_hash(hash: &str) -> Result<&str, HashInvariantError> {
        if hash.is_empty() {
            Err(HashInvariantError::Empty)
        } else if hash.chars().any(|char| !char.is_ascii_hexdigit()) {
            Err(HashInvariantError::NotHexadecimalFormat(hash.to_string()))
        } else if hash.len() != HASH_EXPECTED_LENGTH {
            Err(HashInvariantError::WrongLength(
                HASH_EXPECTED_LENGTH,
                hash.len(),
            ))
        } else {
            Ok(hash)
        }
    }

    fn check_date(date: &str) -> Result<&str, DateInvariantError> {
        if date.is_empty() {
            Err(DateInvariantError::Empty)
        } else {
            Ok(date)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommitData {
    author: String,
    summary: String,
}

impl CommitData {
    pub fn new(author: &str, summary: &str) -> Result<Self, DataInvariantError> {
        Ok(Self {
            author: Self::check_author(author.trim())?.into(),
            summary: Self::check_summary(summary.trim())?.into(),
        })
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    fn check_author(author: &str) -> Result<&str, AuthorInvariantError> {
        if author.is_empty() {
            Err(AuthorInvariantError::Empty)
        } else {
            Ok(author)
        }
    }

    fn check_summary(summary: &str) -> Result<&str, SummaryInvariantError> {
        if summary.is_empty() {
            Err(SummaryInvariantError::Empty)
        } else {
            Ok(summary)
        }
    }
}
