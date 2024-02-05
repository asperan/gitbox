use clap::Args;
use std::rc::Rc;

use crate::{
    application::controller::{exit_code::ControllerExitCode, refresh::RefreshController},
    infrastructure::{
        git_cli::GitCli, gitextra_manager_impl::GitExtraManagerImpl,
        output_manager_impl::OutputManagerImpl, subcommand::Subcommand,
    },
};

#[derive(Args, Clone, Debug)]
#[command(about = "Refresh the content of the git extra folder")]
pub struct RefreshExtraSubcommand {}

impl Subcommand for RefreshExtraSubcommand {
    fn execute(&self) -> i32 {
        let git_cli = Rc::new(GitCli::new());
        let gitextra_manager = Rc::new(GitExtraManagerImpl::new(git_cli.clone()));
        let output_manager = Rc::new(OutputManagerImpl::new());
        let controller = RefreshController::new(
            git_cli.clone(),
            gitextra_manager.clone(),
            output_manager.clone(),
        );
        match controller.refresh() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
