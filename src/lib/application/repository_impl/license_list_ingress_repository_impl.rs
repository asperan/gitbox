use crate::{
    application::manager::license_list_ingress_manager::LicenseListIngressManager,
    usecase::{
        license_metadata::LicenseMetadata,
        repository::license_list_ingress_repository::LicenseListIngressRepository,
        type_aliases::AnyError,
    },
};

pub struct LicenseListIngressRepositoryImpl<'a> {
    license_list_ingress_manager: &'a dyn LicenseListIngressManager,
}

impl<'a, 'b: 'a> LicenseListIngressRepositoryImpl<'a> {
    pub fn new(license_list_ingress_manager: &'b dyn LicenseListIngressManager) -> Self {
        LicenseListIngressRepositoryImpl {
            license_list_ingress_manager,
        }
    }
}

impl LicenseListIngressRepository for LicenseListIngressRepositoryImpl<'_> {
    fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError> {
        self.license_list_ingress_manager.license_list()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::{
            manager::license_list_ingress_manager::LicenseListIngressManager,
            repository_impl::license_list_ingress_repository_impl::LicenseListIngressRepositoryImpl,
        },
        usecase::{
            license_metadata::LicenseMetadata,
            repository::license_list_ingress_repository::LicenseListIngressRepository,
            type_aliases::AnyError,
        },
    };

    struct MockLicenseListIngressManager {
        list: Vec<LicenseMetadata>,
    }
    impl LicenseListIngressManager for MockLicenseListIngressManager {
        fn license_list(&self) -> Result<Box<[LicenseMetadata]>, AnyError> {
            Ok(self.list.as_slice().into())
        }
    }

    #[test]
    fn license_list_ok() {
        let list = [
            LicenseMetadata::new("MIT", "mit"),
            LicenseMetadata::new("Apache2", "apache2"),
        ];
        let manager = MockLicenseListIngressManager {
            list: list.clone().into(),
        };
        let repository = LicenseListIngressRepositoryImpl::new(&manager);
        let result = repository.license_list();
        assert!(result.is_ok_and(
            |it| it.iter().all(|e| list.contains(e)) && list.iter().all(|e| it.contains(e))
        ));
    }
}
