use clap::{ValueEnum, builder::PossibleValue};
use chrono::prelude::*;

use crate::common::{command_issuer::CommandIssuer, commons::print_cli_error_message_and_exit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataSpecs {
    Sha,
    Date,
}

impl ValueEnum for MetadataSpecs {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Sha => Some(PossibleValue::new("sha")),
            Self::Date => Some(PossibleValue::new("date")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Date, Self::Sha]
    }
}

#[derive(Debug)]
pub struct MetadataGenerator {}

impl MetadataGenerator {
    pub fn generate<'a>(specs: impl IntoIterator<Item = &'a MetadataSpecs>) -> Option<String> {
        let mut unique_specs: Vec<MetadataSpecs> = Vec::with_capacity(MetadataSpecs::value_variants().len());
        specs.into_iter().for_each(|s| if !unique_specs.contains(&s) { unique_specs.push(*s); });
        if unique_specs.is_empty() {
            None
        } else {
            unique_specs.iter().map(|s| MetadataGenerator::spec_to_string(&s)).reduce(|accumulator, element| accumulator + "-" + &element)
        }
    }

    fn spec_to_string(spec: &MetadataSpecs) -> String {
        match spec {
            MetadataSpecs::Date => {
                let local_datetime = Local::now();
                local_datetime.format("%F").to_string()
            },
            MetadataSpecs::Sha => {
                let last_commit_sha = CommandIssuer::git(&["--no-pager", "log", "-n", "1", "--pretty=%h"]);
                if last_commit_sha.status.success() {
                    std::str::from_utf8(&last_commit_sha.stdout).unwrap().trim().to_owned()
                } else {
                    print_cli_error_message_and_exit(&last_commit_sha.stderr, "retrieve last commit SHA")
                }
            },
        }
    }
}
