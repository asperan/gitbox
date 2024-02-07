use requestty::{prompt_one, Answer, Question};

use crate::{
    application::manager::license_choice_ingress_manager::LicenseChoiceIngressManager,
    usecase::{license_metadata::LicenseMetadata, type_aliases::AnyError},
};

pub struct LicensePromptHelper {}

impl LicensePromptHelper {
    pub fn new() -> Self {
        LicensePromptHelper {}
    }
}

impl LicenseChoiceIngressManager for LicensePromptHelper {
    fn ask_license<'a>(
        &self,
        list: &'a [LicenseMetadata],
    ) -> Result<&'a LicenseMetadata, AnyError> {
        let choice_list = list.iter().map(|t| t.name());
        let answer = prompt_one(
            Question::raw_select("license")
                .message("Choose a license:")
                .choices(choice_list)
                .build(),
        );
        let answer_index = match answer? {
            Answer::ListItem(choice) => choice.index,
            _ => panic!("Obtained non ListItem from a raw_select"),
        };
        Ok(&list[answer_index])
    }
}
