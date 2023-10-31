use std::str::from_utf8;

use clap::Args;
use regex::Regex;

use crate::common::{
    command_issuer::CommandIssuer,
    commons::{print_cli_error_message_and_exit, print_error_and_exit},
    git::is_in_git_repository,
};

const TREE_FORMAT: &str = "%C(bold blue)%h%C(reset)§%C(dim normal)(%cr)%C(reset)§%C(auto)%d%C(reset)§§%n§§§       %C(normal)%an%C(reset)%C(dim normal): %s%C(reset)";
const TIME_MINIMUM_PADDING: usize = 2;

#[derive(Args, Debug)]
#[command(about = "Print a fancy view of the commit tree")]
pub struct TreeSubCommand {}

impl TreeSubCommand {
    pub fn print_tree(&self) {
        if !is_in_git_repository() {
            print_error_and_exit("tree must be run inside a git repository");
        }
        let git_log_result = CommandIssuer::git([
            "log",
            "--all",
            "--graph",
            "--decorate=short",
            "--date-order",
            "--color",
            &format!("--pretty=format:{}", TREE_FORMAT),
        ]);
        if !&git_log_result.status.success() {
            print_cli_error_message_and_exit(&git_log_result.stderr, "retrieve commit tree");
        }
        let log_raw_output = git_log_result.stdout;
        if log_raw_output.is_empty() {
            println!("This repository has no commits.");
        } else {
            println!("{}", &self.transform_log_output(log_raw_output));
        }
    }

    fn transform_log_output(&self, output: Vec<u8>) -> String {
        let time_regex = Regex::new("\\([a-z0-9 ,]+\\)").unwrap();
        let lines: Vec<Vec<&str>> = from_utf8(&output)
            .unwrap_or("Failed to unwrap output")
            .split('\n')
            .map(|line| line.split('§').collect())
            .collect();

        let time_color_length = {
            let first_line_time = &lines[0][1];
            first_line_time.len()
                - time_regex
                    .captures(first_line_time)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .len()
        };

        let time_padding = lines
            .iter()
            .filter(|line| !line[1].is_empty())
            .map(|line| line[1].len() - time_color_length)
            .max()
            .expect("The calculation of the time color escape sequence is after the length check.");

        lines
            .iter()
            .map(|line| {
                let left_padding = TIME_MINIMUM_PADDING
                    + time_padding
                    + if !line[1].is_empty() && time_regex.is_match(line[1]) {
                        time_color_length
                    } else {
                        0
                    };
                format!(
                    "{date:>width$} {tree_mark} {pointers} {commit_text}\n",
                    date = if line[1].is_empty() { "" } else { line[1] },
                    width = left_padding,
                    tree_mark = line[0],
                    pointers = line[2],
                    commit_text = line[3]
                )
            })
            .collect()
    }
}
