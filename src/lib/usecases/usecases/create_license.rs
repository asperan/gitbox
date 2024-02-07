use crate::usecases::{
    repository::{
        license_choice_ingress_repository::LicenseChoiceIngressRepository,
        license_list_ingress_repository::LicenseListIngressRepository,
        license_text_egress_repository::LicenseTextEgressRepository,
        license_text_ingress_repository::LicenseTextIngressRepository,
    },
    type_aliases::AnyError,
};

use super::usecase::UseCase;

pub struct CreateLicenseUseCase {
    license_list_ingress_repository: Box<dyn LicenseListIngressRepository>,
    license_choice_ingress_repository: Box<dyn LicenseChoiceIngressRepository>,
    license_text_ingress_repository: Box<dyn LicenseTextIngressRepository>,
    license_text_egress_repository: Box<dyn LicenseTextEgressRepository>,
}

impl CreateLicenseUseCase {
    pub fn new(
        license_list_ingress_repository: Box<dyn LicenseListIngressRepository>,
        license_choice_ingress_repository: Box<dyn LicenseChoiceIngressRepository>,
        license_text_ingress_repository: Box<dyn LicenseTextIngressRepository>,
        license_text_egress_repository: Box<dyn LicenseTextEgressRepository>,
    ) -> Self {
        CreateLicenseUseCase {
            license_list_ingress_repository,
            license_choice_ingress_repository,
            license_text_ingress_repository,
            license_text_egress_repository,
        }
    }
}

impl UseCase<()> for CreateLicenseUseCase {
    fn execute(&self) -> Result<(), AnyError> {
        let license_list = self.license_list_ingress_repository.license_list()?;
        let chosen_license = self
            .license_choice_ingress_repository
            .ask_license(&license_list)?;
        let license_text = self.license_text_ingress_repository.text(chosen_license)?;
        self.license_text_egress_repository.consume(&license_text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_license_usecase() {
        unimplemented!();
    }
}
