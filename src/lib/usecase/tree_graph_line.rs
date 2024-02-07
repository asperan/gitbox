const TREE_FORMAT: &str = "%C(bold blue)%h%C(reset)§%C(dim normal)(%cr)%C(reset)§%C(auto)%d%C(reset)§%n§§§       %C(normal)%an%C(reset)%C(dim normal): %s%C(reset)";
const FIELD_SEPARATOR: char = '§';

#[derive(Debug, PartialEq, Eq)]
pub struct TreeGraphLine {
    date: String,
    tree_mark: String,
    pointers: String,
    message: String,
}

impl TreeGraphLine {
    pub fn new(date: String, tree_mark: String, pointers: String, message: String) -> Self {
        TreeGraphLine {
            date,
            tree_mark,
            pointers,
            message,
        }
    }

    pub const fn format() -> &'static str {
        TREE_FORMAT
    }

    pub const fn separator() -> char {
        FIELD_SEPARATOR
    }

    pub fn date(&self) -> &str {
        &self.date
    }
    pub fn tree_mark(&self) -> &str {
        &self.tree_mark
    }
    pub fn pointers(&self) -> &str {
        &self.pointers
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
