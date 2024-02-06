use std::rc::Rc;

use crate::{
    application::manager::{
        license_choice_ingress_manager::LicenseChoiceIngressManager,
        license_list_ingress_manager::LicenseListIngressManager,
        license_text_egress_manager::LicenseTextEgressManager,
        license_text_ingress_manager::LicenseTextIngressManager,
        message_egress_manager::MessageEgressManager,
    },
    usecases::type_aliases::AnyError,
};

use super::exit_code::ControllerExitCode;

pub struct LicenseController {
    license_list_ingress_manager: Rc<dyn LicenseListIngressManager>,
    license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>,
    license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>,
    license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
    message_egress_manager: Rc<dyn MessageEgressManager>,
}

impl LicenseController {
    pub fn new(
        license_list_ingress_manager: Rc<dyn LicenseListIngressManager>,
        license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>,
        license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>,
        license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
        message_egress_manager: Rc<dyn MessageEgressManager>,
    ) -> Self {
        LicenseController {
            license_list_ingress_manager,
            license_choice_ingress_manager,
            license_text_ingress_manager,
            license_text_egress_manager,
            message_egress_manager,
        }
    }

    pub fn license(&self) -> ControllerExitCode {
        match self.run() {
            Ok(_) => {
                self.message_egress_manager.output("License file created successfully. Remember to change author and year reference.");
                ControllerExitCode::Ok
            }
            Err(e) => {
                self.message_egress_manager
                    .error(&format!("Failed to create license file: {}", e));
                ControllerExitCode::Error(1)
            }
        }
    }

    fn run(&self) -> Result<(), AnyError> {
        let list = self.license_list_ingress_manager.license_list()?;
        let choice = self.license_choice_ingress_manager.ask_license(list)?;
        let text = self.license_text_ingress_manager.license_text(choice)?;
        self.license_text_egress_manager.write_license(&text)
    }
}
