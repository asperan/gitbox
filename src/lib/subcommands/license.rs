use std::path::Path;

use clap::Args;
use regex::{Regex, RegexBuilder};
use requestty::{prompt_one, Answer, Question};

use crate::common::commons::print_error_and_exit;

type NetError = Box<ureq::Error>;

const BASE_LICENSE_URL: &str = "https://choosealicense.com";
const HTML_LICENSE_PREFIX: &str = "<pre id=\"license-text\">";
const HTML_LICENSE_SUFFIX: &str = "</pre>";

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

impl LicenseSubCommand {
    pub fn create_license(&self) {
        let license_url = self.ask_for_license_url();
        let license_text = self.download_license(&license_url);
        match license_text {
            Ok(text) => {
                self.write_to_file(&self.filename, &text);
            }
            Err(e) => print_error_and_exit(&format!(
                "Failed to downlad license text: {}",
                &e.to_string()
            )),
        }
    }

    fn ask_for_license_url(&self) -> String {
        let list = self.download_license_list();
        match list {
            Ok(l) => {
                let choice_list: Vec<&String> = l.iter().map(|t| &t.1).collect();
                let answer = prompt_one(
                    Question::raw_select("license")
                        .message("Choose a license:")
                        .choices(choice_list)
                        .build(),
                );
                let answer_index = match answer {
                    Ok(Answer::ListItem(choice)) => choice.index,
                    Ok(_) => panic!("Obtained non ListItem from a raw_select"),
                    Err(e) => print_error_and_exit(&e.to_string()),
                };
                l[answer_index].0.clone()
            }
            Err(e) => {
                print_error_and_exit(&format!("Failed to get license list: {}", &e.to_string()))
            }
        }
    }

    fn download_license_list(&self) -> Result<Vec<(String, String)>, NetError> {
        let list_url = BASE_LICENSE_URL.to_owned() + "/appendix";
        let raw_list = ureq::get(&list_url).call()?.into_string();
        match raw_list {
            Ok(s) => {
                let url_and_name_regex = Regex::new("<a href=\"(.*)\">(.*)</a>").unwrap();
                let raw_licenses: Vec<(String, String)> = s
                    .split('\n')
                    .filter(|line| line.contains("<th scope=\"row\">"))
                    .filter_map(|line| {
                        let captures = url_and_name_regex.captures(line);
                        captures.map(|c| {
                            (
                                c.get(1).unwrap().as_str().to_owned(),
                                c.get(2).unwrap().as_str().to_owned(),
                            )
                        })
                    })
                    .collect();
                Ok(raw_licenses)
            }
            Err(e) => Err(Box::new(e.into())),
        }
    }

    fn download_license(&self, url_path: &str) -> Result<String, NetError> {
        let license_url = BASE_LICENSE_URL.to_owned() + url_path;
        let raw_license_page = ureq::get(&license_url).call()?.into_string();
        match raw_license_page {
            Ok(page) => {
                let license_text_regex = RegexBuilder::new(&format!(
                    "{}(.*){}",
                    HTML_LICENSE_PREFIX, HTML_LICENSE_SUFFIX
                ))
                .dot_matches_new_line(true)
                .build()
                .unwrap();
                let raw_license_text = license_text_regex.find(&page);
                match raw_license_text {
                    Some(text_match) => Ok(text_match
                        .as_str()
                        .trim_start_matches(HTML_LICENSE_PREFIX)
                        .trim_end_matches(HTML_LICENSE_SUFFIX)
                        .trim()
                        .to_owned()),
                    None => print_error_and_exit("Failed to parse license text"),
                }
            }
            Err(e) => Err(Box::new(e.into())),
        }
    }

    fn write_to_file(&self, filename: &str, content: &str) {
        let relative_filepath = "./".to_owned() + filename;
        let filepath = Path::new(&(relative_filepath));
        let write_result = std::fs::write(filepath, content);
        match write_result {
            Ok(()) => println!("License file created successfully. Remember to update the date and the owner of the copyright."),
            Err(e) => print_error_and_exit(&e.to_string()),
        }
    }
}
