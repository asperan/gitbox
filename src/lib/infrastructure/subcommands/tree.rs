use clap::Args;

use crate::{
    application::controller::{exit_code::ControllerExitCode, tree::TreeController},
    infrastructure::{
        interface::{git_cli::GitCli, message_egress_manager_impl::MessageEgressManagerImpl},
        subcommand::Subcommand,
    },
};

#[derive(Args, Debug)]
#[command(about = "Print a fancy view of the commit tree")]
pub struct TreeSubCommand {
    #[arg(
        long,
        default_value = "false",
        help = "Set whether to use the default color behaviour with pipes and redirections"
    )]
    use_default_color_behaviour: bool,
}

impl Subcommand for TreeSubCommand {
    fn execute(&self) -> i32 {
        if self.use_default_color_behaviour {
            colored::control::unset_override();
        } else {
            colored::control::set_override(true);
        }
        let git_cli = GitCli::new();
        let message_egress_manager = MessageEgressManagerImpl::new();
        let controller = TreeController::new(&git_cli, &message_egress_manager);
        match controller.commit_tree() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
