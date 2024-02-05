use std::rc::Rc;

use clap::Args;

use crate::{
    application::{
        controller::{exit_code::ControllerExitCode, init::InitController},
        manager::message_egress_manager::MessageEgressManager,
        options::init::InitOptions,
        manager::gitinfo_ingress_manager::GitInfoIngressManager,
    },
    infrastructure::{
        git_cli::GitCli, message_egress_manager_impl::MessageEgressManagerImpl, subcommand::Subcommand,
    },
};

#[derive(Args, Debug)]
#[command(about = "Initialize a git repository")]
pub struct InitSubCommand {
    #[arg(short, long, help = "Do not create the first, empty commit")]
    empty: bool,
}

impl Subcommand for InitSubCommand {
    fn execute(&self) -> i32 {
        let output_manager = Rc::new(MessageEgressManagerImpl::new());
        let git_cli = Rc::new(GitCli::new());
        let gitinfo_retriever = git_cli.clone();
        if gitinfo_retriever.git_dir().is_ok() {
            output_manager.error("init subcommand cannot be called inside a git dir");
            return 1;
        }
        let options = InitOptions::new(self.empty);
        let controller = InitController::new(
            options,
            git_cli.clone(),
            git_cli.clone(),
            output_manager.clone(),
        );
        match controller.init() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
