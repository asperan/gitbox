use std::process::{Output, Command};
use crate::common::commons::print_error_and_exit;

#[derive(Debug)]
pub struct CommandIssuer { }

impl CommandIssuer {
    pub fn run(executable: &str, args: &[&str]) -> Output {
        Command::new(executable)
                .args(args)
                .output()
                .unwrap_or_else( |e| {
                    print_error_and_exit(&format!("Failed to run command '{}': {}", &args.join(" "), e));
                })
    }

    pub fn git(args: &[&str]) -> Output {
        CommandIssuer::run("git", args)
    }
}
