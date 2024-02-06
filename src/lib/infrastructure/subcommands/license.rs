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
        let license_scraper = Rc::new(LicenseDownloadIngressManagerImpl::new());
        let message_egress_manager = Rc::new(MessageEgressManagerImpl::new());
        let license_choice_ingress_manager = Rc::new(LicensePromptHelper::new());
        let license_text_egress_manager = Rc::new(FileWriter::new());
        let controller = LicenseController::new(
            options,
            license_scraper.clone(),
            license_choice_ingress_manager.clone(),
            license_scraper.clone(),
            license_text_egress_manager.clone(),
            message_egress_manager.clone(),
        );
        match controller.license() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }
}
