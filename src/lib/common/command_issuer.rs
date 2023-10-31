use crate::common::commons::print_error_and_exit;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct CommandIssuer {}

impl<'a> CommandIssuer {
    pub fn run(executable: &str, args: impl IntoIterator<Item = &'a str> + Clone) -> Output {
        Command::new(executable)
            .args(args.clone())
            .output()
            .unwrap_or_else(|e| {
                print_error_and_exit(&format!(
                    "Failed to run command '{} {}': {}",
                    executable,
                    args.into_iter()
                        .fold(String::from(""), |acc, x| acc.to_string() + " " + x),
                    e
                ));
            })
    }

    pub fn git(args: impl IntoIterator<Item = &'a str> + Clone) -> Output {
        CommandIssuer::run("git", args)
    }
}
