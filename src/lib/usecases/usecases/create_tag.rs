use std::rc::Rc;

use crate::usecases::{
    configuration::tag::TagConfiguration, repository::tag_write_repository::TagWriteRepository,
    type_aliases::AnyError,
};

use super::usecase::UseCase;

pub struct CreateTagUseCase {
    configuration: TagConfiguration,
    tag_write_repository: Rc<dyn TagWriteRepository>,
}

impl CreateTagUseCase {
    pub fn new(
        configuration: TagConfiguration,
        tag_write_repository: Rc<dyn TagWriteRepository>,
    ) -> CreateTagUseCase {
        CreateTagUseCase {
            configuration,
            tag_write_repository,
        }
    }
}

impl UseCase<()> for CreateTagUseCase {
    fn execute(&self) -> Result<(), AnyError> {
        self.tag_write_repository.create_tag(
            self.configuration.version(),
            self.configuration.message(),
            self.configuration.sign(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, cell::RefCell, rc::Rc};

    use crate::{
        domain::semantic_version::SemanticVersion,
        usecases::{
            configuration::tag::TagConfiguration,
            repository::tag_write_repository::TagWriteRepository, type_aliases::AnyError,
            usecases::usecase::UseCase,
        },
    };

    use super::CreateTagUseCase;

    struct MockTagWriteRepository {
        version: RefCell<SemanticVersion>,
        message: RefCell<Option<String>>,
        sign: RefCell<bool>,
    }

    impl MockTagWriteRepository {
        pub fn new() -> MockTagWriteRepository {
            MockTagWriteRepository {
                version: RefCell::new(SemanticVersion::first_release()),
                message: RefCell::new(None),
                sign: RefCell::new(false),
            }
        }
    }

    impl TagWriteRepository for MockTagWriteRepository {
        fn create_tag(
            &self,
            version: &SemanticVersion,
            message: &Option<String>,
            sign: bool,
        ) -> Result<(), AnyError> {
            self.version.replace(version.clone());
            self.message.replace(message.clone());
            self.sign.replace(sign);
            Ok(())
        }
    }

    #[test]
    fn usecase_propagate_configuration() {
        let tag_configuration = TagConfiguration::new(
            SemanticVersion::new(1, 0, 0, None, None),
            Some("test".to_string()),
            true,
        );
        let tag_write_repository = Rc::new(MockTagWriteRepository::new());
        let usecase = CreateTagUseCase::new(tag_configuration, tag_write_repository.clone());
        let _result = usecase.execute().expect("Mock does not return an error");
        assert_eq!(
            tag_write_repository.version.borrow().to_owned(),
            SemanticVersion::new(1, 0, 0, None, None)
        );
        assert_eq!(
            tag_write_repository.message.borrow().to_owned(),
            Some("test".to_string())
        );
        assert_eq!(tag_write_repository.sign.borrow().to_owned(), true);
    }
}