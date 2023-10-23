use crate::common::command_issuer::CommandIssuer;

use super::commons::print_error_and_exit;

pub fn is_in_git_repository() -> bool {
    CommandIssuer::git(vec!("rev-parse", "--is-inside-work-tree")).status.success()
}

pub fn git_dir() -> String {
    match std::str::from_utf8(&CommandIssuer::git(vec!["rev-parse", "--absolute-git-dir"]).stdout) {
        Ok(path) => path.trim().to_string(),
        Err(e) => print_error_and_exit(&e.to_string()),
    }
}

pub const EXTRA_DIR_PATH: &str = "/extra";
pub const TYPES_FILE_PATH: &str = "/types.txt";
pub const SCOPES_FILE_PATH: &str = "/scopes.txt";

// Groups: 1 = type, 2 = scope with (), 3 = scope, 4 = breaking change, 5 = summary
pub const CONVENTIONAL_COMMIT_REGEX: &str = r"^(\w+)(\(([\w/-]+)\))?(!)?:(.*)";

pub const DEFAULT_COMMIT_TYPES: [&str; 10] = [
    "feat",
    "fix",
    "build",
    "chore",
    "ci",
    "docs",
    "style",
    "refactor",
    "perf",
    "test",
];
