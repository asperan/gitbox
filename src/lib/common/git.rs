use crate::common::command_issuer::CommandIssuer;

use super::{commons::print_error_and_exit, cached_values::CachedValues};

pub fn is_in_git_repository() -> bool {
    CommandIssuer::git(&["rev-parse", "--is-inside-work-tree"]).status.success()
}

pub(super) fn git_dir() -> String {
    match std::str::from_utf8(&CommandIssuer::git(&["rev-parse", "--absolute-git-dir"]).stdout) {
        Ok(path) => path.trim().to_string(),
        Err(e) => print_error_and_exit(&e.to_string()),
    }
}

pub const EXTRA_DIR_PATH: &str = "/extra";
pub const TYPES_FILE_PATH: &str = "/types.txt";
pub const SCOPES_FILE_PATH: &str = "/scopes.txt";

// Groups: 1 = type, 2 = scope with (), 3 = scope, 4 = breaking change, 5 = summary
pub(super) const CONVENTIONAL_COMMIT_PATTERN: &str = r"^(\w+)(\(([\w/-]+)\))?(!)?:(.+)$";

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

pub const FIRST_STABLE_RELEASE: &str = "0.1.0";
pub(super) const FULL_SEMANTIC_VERSION_PATTERN: &str = concat!(
    // GROUPS:
    // 1 = Stable version, 2 = major, 3 = minor, 4 = patch
    r"^((0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*))",
    // 5 = prerelease
    r"(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?",
    // 6 = metadata
    r"(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"
);

pub(super) fn last_version() -> Option<String> {
    let result = CommandIssuer::git(&["describe", "--tags", "--abbrev=0"]);
    if result.status.success() {
        match std::str::from_utf8(&result.stdout) {
            Ok(version) => Some(version.trim().to_owned()),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    } else {
        None
    }
}

pub(super) fn last_release() -> Option<String> {
    let result = CommandIssuer::git(&["--no-pager", "tag", "--list", "--merged"]);
    if result.status.success() {
        match std::str::from_utf8(&result.stdout) {
            Ok(text) => {
                if text.trim().is_empty() {
                    None
                } else {
                    let mut to_sort_versions: Vec<String> = text.trim()
                        .split('\n')
                        .filter(|version|
                            CachedValues::semantic_version_regex().captures(version).is_some_and(|captures| captures.get(5).is_none())
                        ).map(|version| version.replace('+', "_"))
                        .collect();
                    to_sort_versions.sort_unstable();
                    to_sort_versions.last().map(|s| s.replace('_', "+").to_owned())
                }
            },
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    } else {
        None
    }
}

pub fn commit_list(from: Option<&String>) -> Vec<String> {
    let result = match from {
        Some(value) => CommandIssuer::git(&[ "--no-pager", "log", "--oneline", "--pretty=format:%s", &format!("^{} HEAD", value)]),
        None => CommandIssuer::git(&[ "--no-pager", "log", "--oneline", "--pretty=format:%s"]),
    };
    if result.status.success() {
        match std::str::from_utf8(&result.stdout) {
            Ok(lines) => {
                lines.split('\n').map(|s| s.to_string()).collect()
            },
            Err(e) => print_error_and_exit(&e.to_string())
        }
    } else {
        print_error_and_exit("Failed to retrieve commit list")
    }
}
