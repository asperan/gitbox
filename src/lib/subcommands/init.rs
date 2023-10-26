use clap::Args;

use crate::common::{
    command_issuer::CommandIssuer,
    commons::{print_cli_error_message_and_exit, print_error_and_exit},
    git::is_in_git_repository,
};

#[derive(Args, Debug)]
#[command(about = "Initialize a git repository")]
pub struct InitSubCommand {
    #[arg(short, long, help = "Do not create the first, empty commit")]
    empty: bool,
}

impl InitSubCommand {
    pub fn init_repository(&self) {
        if is_in_git_repository() {
            print_error_and_exit("The current directory is already a git repository");
        }
        let init_output = CommandIssuer::git(&["init"]);
        if !init_output.status.success() {
            print_cli_error_message_and_exit(&init_output.stderr, "initialize repository");
        }
        if !self.empty {
            let commit_output = CommandIssuer::git(&[
                "commit",
                "--allow-empty",
                "-m",
                "chore: initialize repository",
            ]);
            if !commit_output.status.success() {
                print_cli_error_message_and_exit(&commit_output.stderr, "create the first commit");
            }
        }
    }
}
