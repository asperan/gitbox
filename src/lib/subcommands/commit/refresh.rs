use clap::Args;
use std::io::Write;

use crate::common::{
    cached_values::CachedValues,
    command_issuer::CommandIssuer,
    commons::{ensure_dir_exists, print_cli_error_message_and_exit, print_error_and_exit},
    git::{DEFAULT_COMMIT_TYPES, EXTRA_DIR_PATH, SCOPES_FILE_PATH, TYPES_FILE_PATH},
};

#[derive(Args, Clone, Debug)]
pub struct RefreshTypesAndScopesSubcommand {}

impl RefreshTypesAndScopesSubcommand {
    pub fn refresh_types_and_scopes(&self) {
        let conventional_commit_regex = CachedValues::conventional_commit_regex();
        let mut all_types = DEFAULT_COMMIT_TYPES.map(|t| t.to_string()).to_vec();
        let mut all_scopes: Vec<String> = vec![];
        self.full_commit_list()
            .iter()
            .filter_map(|commit| conventional_commit_regex.captures(commit))
            .for_each(|capture| {
                match capture.get(1) {
                    Some(m) if !all_types.contains(&m.as_str().to_string()) => {
                        all_types.push(m.as_str().to_string())
                    }
                    _ => {}
                }
                match capture.get(3) {
                    Some(m) if !all_scopes.contains(&m.as_str().to_string()) => {
                        all_scopes.push(m.as_str().to_string())
                    }
                    _ => {}
                }
            });
        ensure_dir_exists(&(CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH));
        self.rewrite_file(
            &(CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + TYPES_FILE_PATH),
            &all_types.join("\n"),
        );
        self.rewrite_file(
            &(CachedValues::git_dir().to_owned() + EXTRA_DIR_PATH + SCOPES_FILE_PATH),
            &all_scopes.join("\n"),
        );
    }

    fn full_commit_list(&self) -> Vec<String> {
        let result = CommandIssuer::git(&["log", "--all", "--reverse", "--pretty=format:%s"]);
        if result.status.success() {
            match std::str::from_utf8(&result.stdout) {
                Ok(s) => s.split('\n').map(|s| s.to_string()).collect(),
                Err(e) => print_error_and_exit(&e.to_string()),
            }
        } else {
            print_cli_error_message_and_exit(&result.stderr, "obtain commit list")
        }
    }

    fn rewrite_file(&self, file_path: &str, content: &str) {
        let file = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path);
        match file {
            Ok(mut f) => {
                if let Err(e) = writeln!(&mut f, "{}", content) {
                    eprintln!("Failed to update file '{}': {}", file_path, e)
                }
            }
            Err(e) => eprintln!("Failed to open file {}: {}", file_path, e),
        }
    }
}
