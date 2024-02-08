use std::rc::Rc;

use clap::Args;

use crate::{
    application::{
        controller::{exit_code::ControllerExitCode, license::LicenseController},
        options::license::LicenseOptions,
    },
    infrastructure::{
        helper::license_prompt_helper::LicensePromptHelper,
        interface::{
            file_writer::FileWriter,
            license_download_ingress_manager_impl::LicenseDownloadIngressManagerImpl,
            message_egress_manager_impl::MessageEgressManagerImpl,
        },
        subcommand::Subcommand,
    },
};

#[derive(Args, Debug)]
#[command(about = "Create a license file")]
pub struct LicenseSubCommand {
    #[arg(
        short,
        long,
        default_value = "LICENSE",
        help = "Set the license file name"
    )]
    filename: String,
}

impl Subcommand for LicenseSubCommand {
    fn execute(&self) -> i32 {
        let options = LicenseOptions::new(&self.filename);
        let license_scraper = LicenseDownloadIngressManagerImpl::new();
        let message_egress_manager = MessageEgressManagerImpl::new();
        let license_choice_ingress_manager = LicensePromptHelper::new();
        let license_text_egress_manager = FileWriter::new();
        let controller = LicenseController::new(
            options,
            &license_scraper,
            &license_choice_ingress_manager,
            &license_scraper,
            &license_text_egress_manager,
            &message_egress_manager,
        );
        match controller.license() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
