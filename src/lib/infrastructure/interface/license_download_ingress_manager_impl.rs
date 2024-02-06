use regex::{Regex, RegexBuilder};

use crate::{
    application::{
        manager::{
            license_list_ingress_manager::LicenseListIngressManager,
            license_text_ingress_manager::LicenseTextIngressManager,
        },
        type_alias::LicenseNameAndId,
    },
    infrastructure::error::license_text_retrieval_error::LicenseTextRetrievalError,
    usecases::type_aliases::AnyError,
};

const BASE_LICENSE_URL: &str = "https://choosealicense.com";
const HTML_LICENSE_PREFIX: &str = "<pre id=\"license-text\">";
const HTML_LICENSE_SUFFIX: &str = "</pre>";

pub struct LicenseDownloadIngressManagerImpl {}

impl LicenseDownloadIngressManagerImpl {
    pub fn new() -> Self {
        LicenseDownloadIngressManagerImpl {  }
    }
}

impl LicenseListIngressManager for LicenseDownloadIngressManagerImpl {
    fn license_list(&self) -> Result<Box<[LicenseNameAndId]>, AnyError> {
        let list_url = BASE_LICENSE_URL.to_owned() + "/appendix";
        let raw_list = ureq::get(&list_url).call()?.into_string()?;
        let url_and_name_regex = Regex::new("<a href=\"(.*)\">(.*)</a>").unwrap();
        Ok(raw_list
            .split('\n')
            .filter(|line| line.contains("<th scope=\"row\">"))
            .filter_map(|line| {
                let captures = url_and_name_regex.captures(line);
                captures.map(|c| {
                    LicenseNameAndId(
                        c.get(1).unwrap().as_str().into(),
                        c.get(2).unwrap().as_str().into(),
                    )
                })
            })
            .collect())
    }
}

impl LicenseTextIngressManager for LicenseDownloadIngressManagerImpl {
    fn license_text(&self, license: Box<LicenseNameAndId>) -> Result<Box<str>, AnyError> {
        let license_url = BASE_LICENSE_URL.to_owned() + &license.1;
        let raw_license_page = ureq::get(&license_url).call()?.into_string()?;
        let license_text_regex = RegexBuilder::new(&format!(
            "{}(.*){}",
            HTML_LICENSE_PREFIX, HTML_LICENSE_SUFFIX
        ))
        .dot_matches_new_line(true)
        .build()
        .unwrap();
        let raw_license_text = license_text_regex.find(&raw_license_page);
        match raw_license_text {
            Some(text_match) => Ok(text_match
                .as_str()
                .trim_start_matches(HTML_LICENSE_PREFIX)
                .trim_end_matches(HTML_LICENSE_SUFFIX)
                .trim()
                .into()),
            None => Err(Box::new(LicenseTextRetrievalError::new(
                "failed to parse license text",
            ))),
        }
    }
}
