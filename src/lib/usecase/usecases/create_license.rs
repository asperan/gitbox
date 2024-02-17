use crate::usecase::{
    repository::{
        license_choice_ingress_repository::LicenseChoiceIngressRepository,
        license_list_ingress_repository::LicenseListIngressRepository,
        license_text_egress_repository::LicenseTextEgressRepository,
        license_text_ingress_repository::LicenseTextIngressRepository,
    },
    type_aliases::AnyError,
};

use super::usecase::UseCase;

pub struct CreateLicenseUseCase<'a> {
    license_list_ingress_repository: &'a dyn LicenseListIngressRepository,
    license_choice_ingress_repository: &'a dyn LicenseChoiceIngressRepository,
    license_text_ingress_repository: &'a dyn LicenseTextIngressRepository,
    license_text_egress_repository: &'a dyn LicenseTextEgressRepository,
}

impl<'a, 'b: 'a, 'c: 'a, 'd: 'a, 'e: 'a> CreateLicenseUseCase<'a> {
    pub fn new(
        license_list_ingress_repository: &'b dyn LicenseListIngressRepository,
        license_choice_ingress_repository: &'c dyn LicenseChoiceIngressRepository,
        license_text_ingress_repository: &'d dyn LicenseTextIngressRepository,
        license_text_egress_repository: &'e dyn LicenseTextEgressRepository,
    ) -> Self {
        CreateLicenseUseCase {
            license_list_ingress_repository,
            license_choice_ingress_repository,
            license_text_ingress_repository,
            license_text_egress_repository,
        }
    }
}

impl UseCase<()> for CreateLicenseUseCase<'_> {
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
    use std::cell::RefCell;

    use crate::usecase::{
        license_metadata::LicenseMetadata,
        repository::{
            license_choice_ingress_repository::LicenseChoiceIngressRepository,
            license_list_ingress_repository::LicenseListIngressRepository,
            license_text_egress_repository::LicenseTextEgressRepository,
            license_text_ingress_repository::LicenseTextIngressRepository,
        },
        type_aliases::AnyError,
        usecases::{create_license::CreateLicenseUseCase, usecase::UseCase},
    };

    struct MockLicenseListIngressRepository {}
    impl LicenseListIngressRepository for MockLicenseListIngressRepository {
        fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError> {
            Ok(Box::new([
                LicenseMetadata::new("MIT", "mit-license"),
                LicenseMetadata::new("MPL 2.0", "mpl-2.0"),
            ]))
        }
    }

    struct MockLicenseChoiceIngressRepository {}
    impl LicenseChoiceIngressRepository for MockLicenseChoiceIngressRepository {
        fn ask_license<'a>(
            &self,
            list: &'a [LicenseMetadata],
        ) -> Result<&'a LicenseMetadata, AnyError> {
            Ok(&list[0])
        }
    }

    struct MockLicenseTextIngressRepository {}
    impl LicenseTextIngressRepository for MockLicenseTextIngressRepository {
        fn text(&self, license: &LicenseMetadata) -> Result<Box<str>, AnyError> {
            Ok(format!(
                "Name: {}\nReference: {}\n",
                license.name(),
                license.reference()
            )
            .into_boxed_str())
        }
    }

    struct MockLicenseTextEgressRepository {
        consumed_text: RefCell<Box<str>>,
    }

    impl LicenseTextEgressRepository for MockLicenseTextEgressRepository {
        fn consume(&self, text: &str) -> Result<(), AnyError> {
            self.consumed_text.replace(text.into());
            Ok(())
        }
    }

    #[test]
    fn create_license_usecase() {
        let license_list_ingress_repository = MockLicenseListIngressRepository {};
        let license_choice_ingress_repository = MockLicenseChoiceIngressRepository {};
        let license_text_ingress_repository = MockLicenseTextIngressRepository {};
        let license_text_egress_repository = MockLicenseTextEgressRepository {
            consumed_text: RefCell::new("".into()),
        };
        let usecase = CreateLicenseUseCase::new(
            &license_list_ingress_repository,
            &license_choice_ingress_repository,
            &license_text_ingress_repository,
            &license_text_egress_repository,
        );
        usecase.execute().expect("Repositories do not return Errs");
        assert_eq!(
            license_text_egress_repository.consumed_text.take(),
            "Name: MIT\nReference: mit-license\n".into()
        );
    }
}
