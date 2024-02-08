use std::rc::Rc;

use crate::{
    application::{
        manager::{
            license_choice_ingress_manager::LicenseChoiceIngressManager,
            license_list_ingress_manager::LicenseListIngressManager,
            license_text_egress_manager::LicenseTextEgressManager,
            license_text_ingress_manager::LicenseTextIngressManager,
            message_egress_manager::MessageEgressManager,
        },
        options::license::LicenseOptions,
        repository_impl::{
            license_choice_ingress_repository_impl::LicenseChoiceIngressRepositoryImpl,
            license_list_ingress_repository_impl::LicenseListIngressRepositoryImpl,
            license_text_egress_repository_impl::LicenseTextEgressRepositoryImpl,
            license_text_ingress_repository_impl::LicenseTextIngressRepositoryImpl,
        },
    },
    usecase::usecases::{create_license::CreateLicenseUseCase, usecase::UseCase},
};

use super::exit_code::ControllerExitCode;

pub struct LicenseController {
    options: LicenseOptions,
    license_list_ingress_manager: Rc<dyn LicenseListIngressManager>,
    license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>,
    license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>,
    license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
    message_egress_manager: Rc<dyn MessageEgressManager>,
}

impl LicenseController {
    pub fn new(
        options: LicenseOptions,
        license_list_ingress_manager: Rc<dyn LicenseListIngressManager>,
        license_choice_ingress_manager: Rc<dyn LicenseChoiceIngressManager>,
        license_text_ingress_manager: Rc<dyn LicenseTextIngressManager>,
        license_text_egress_manager: Rc<dyn LicenseTextEgressManager>,
        message_egress_manager: Rc<dyn MessageEgressManager>,
    ) -> Self {
        LicenseController {
            options,
            license_list_ingress_manager,
            license_choice_ingress_manager,
            license_text_ingress_manager,
            license_text_egress_manager,
            message_egress_manager,
        }
    }

    pub fn license(&self) -> ControllerExitCode {
        let license_list_ingress_repository = Rc::new(LicenseListIngressRepositoryImpl::new(
            self.license_list_ingress_manager.clone(),
        ));
        let license_choice_ingress_repository = Rc::new(LicenseChoiceIngressRepositoryImpl::new(
            self.license_choice_ingress_manager.clone(),
        ));
        let license_text_ingress_repository = Rc::new(LicenseTextIngressRepositoryImpl::new(
            self.license_text_ingress_manager.clone(),
        ));
        let license_text_egress_repository = Rc::new(LicenseTextEgressRepositoryImpl::new(
            self.options.path(),
            self.license_text_egress_manager.clone(),
        ));
        let usecase = CreateLicenseUseCase::new(
            license_list_ingress_repository,
            license_choice_ingress_repository,
            license_text_ingress_repository,
            license_text_egress_repository,
        );
        match usecase.execute() {
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
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, fmt::Display, rc::Rc};

    use crate::{
        application::{
            controller::{exit_code::ControllerExitCode, license::LicenseController},
            manager::{
                license_choice_ingress_manager::LicenseChoiceIngressManager,
                license_list_ingress_manager::LicenseListIngressManager,
                license_text_egress_manager::{self, LicenseTextEgressManager},
                license_text_ingress_manager::LicenseTextIngressManager,
                message_egress_manager::MessageEgressManager,
            },
            options::license::LicenseOptions,
        },
        usecase::{license_metadata::LicenseMetadata, type_aliases::AnyError},
    };

    #[derive(Debug)]
    struct MockError {}
    impl Display for MockError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Mock error")
        }
    }
    impl Error for MockError {}

    struct MockLicenseChoiceIngressManager {}

    impl LicenseChoiceIngressManager for MockLicenseChoiceIngressManager {
        fn ask_license<'a>(
            &self,
            list: &'a [LicenseMetadata],
        ) -> Result<&'a LicenseMetadata, AnyError> {
            if !list.is_empty() {
                Ok(&list[0])
            } else {
                Err(MockError {}.into())
            }
        }
    }

    struct MockLicenseListIngressManager {
        list: Vec<LicenseMetadata>,
    }
    impl LicenseListIngressManager for MockLicenseListIngressManager {
        fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError> {
            Ok(self.list.as_slice().into())
        }
    }

    struct MockLicenseTextEgressManager {
        filepath: RefCell<Box<str>>,
        text: RefCell<Box<str>>,
    }
    impl LicenseTextEgressManager for MockLicenseTextEgressManager {
        fn write_license(&self, filepath: &str, text: &str) -> Result<(), AnyError> {
            self.filepath.replace(filepath.into());
            self.text.replace(text.into());
            Ok(())
        }
    }

    struct MockLicenseTextIngressManager {
        text: Box<str>,
    }
    impl LicenseTextIngressManager for MockLicenseTextIngressManager {
        fn license_text(&self, _license: &LicenseMetadata) -> Result<Box<str>, AnyError> {
            Ok(self.text.clone())
        }
    }

    struct VoidMessageEgressManager {}
    impl MessageEgressManager for VoidMessageEgressManager {
        fn output(&self, _message: &str) {}
        fn error(&self, _error: &str) {}
    }

    #[test]
    fn license_controller() {
        let options = LicenseOptions::new("/tmp/test-path");
        let license_list_ingress_manager = Rc::new(MockLicenseListIngressManager {
            list: vec![LicenseMetadata::new("MIT", "mit-license")],
        });
        let license_choice_ingress_manager = Rc::new(MockLicenseChoiceIngressManager {});
        let license_text_ingress_manager = Rc::new(MockLicenseTextIngressManager {
            text: "License text".into(),
        });
        let license_text_egress_manager = Rc::new(MockLicenseTextEgressManager {
            text: RefCell::new("".into()),
            filepath: RefCell::new("".into()),
        });
        let message_egress_manager = Rc::new(VoidMessageEgressManager {});
        let controller = LicenseController::new(
            options,
            license_list_ingress_manager,
            license_choice_ingress_manager,
            license_text_ingress_manager,
            license_text_egress_manager,
            message_egress_manager,
        );
        let result = controller.license();
        assert!(matches!(result, ControllerExitCode::Ok));
    }
}
