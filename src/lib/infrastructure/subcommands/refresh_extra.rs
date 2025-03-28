use clap::Args;

use crate::{
    application::{
        controller::{exit_code::ControllerExitCode, refresh::RefreshController},
        manager::{
            gitinfo_ingress_manager::GitInfoIngressManager,
            message_egress_manager::MessageEgressManager,
        },
    },
    infrastructure::{
        interface::{
            git_cli::GitCli, gitextra_manager_impl::GitExtraManagerImpl,
            message_egress_manager_impl::MessageEgressManagerImpl,
        },
        subcommand::Subcommand,
    },
};

#[derive(Args, Clone, Debug)]
#[command(about = "Refresh the content of the git extra folder")]
pub struct RefreshExtraSubcommand {}

impl Subcommand for RefreshExtraSubcommand {
    fn execute(&self) -> i32 {
        let git_cli = GitCli::new();
        let output_manager = MessageEgressManagerImpl::new();
        if let Err(e) = git_cli.git_dir() {
            output_manager.error(&format!("Failed to retrieve git dir: {}", e));
            output_manager.error("refresh-extra subcommand can only be run inside a git project");
            return 1;
        }
        let gitextra_manager = GitExtraManagerImpl::new(&git_cli);
        let controller = RefreshController::new(&git_cli, &gitextra_manager, &output_manager);
        match controller.refresh() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
