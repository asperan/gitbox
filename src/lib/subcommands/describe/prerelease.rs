use regex::Regex;

use crate::common::{semantic_version::SemanticVersion, commons::print_error_and_exit};

#[derive(Debug)]
pub struct PrereleaseUpdater {
    new_pattern: String,
    old_pattern: String,
}


impl PrereleaseUpdater {
    pub fn new(new_pattern: &str, old_pattern: &str) -> PrereleaseUpdater {
        if !new_pattern.contains("%d") {
            print_error_and_exit("The prerelease new pattern does not contain the placeholder '%d'")
        } else if !old_pattern.contains("%d") {
            print_error_and_exit("The prerelease old pattern does not contain the placeholder '%d'")
        } else {
            PrereleaseUpdater {
                new_pattern: new_pattern.to_owned(),
                old_pattern: old_pattern.to_owned(),
            }
        }
    }

    pub fn update_prerelease(&self, new_version: &SemanticVersion, old_version: &Option<SemanticVersion>) -> String {
        let stable_updated = match &old_version {
            Some(old) => {
                new_version.major() != old.major()
                || new_version.minor() != new_version.minor()
                || new_version.patch() != new_version.patch()
            },
            None => true,
        };
        if self.old_pattern != self.new_pattern || stable_updated || old_version.as_ref().is_some_and(|v| v.prerelease().is_none()){
            self.new_pattern.replace("%d", "1")
        } else {
            let old_pattern_regex = Regex::new(&self.old_pattern.replace("%d", "(\\d+)")).unwrap();
            let number:u16 = old_pattern_regex.captures(old_version.as_ref().unwrap().prerelease().as_ref().unwrap()).unwrap().get(1).unwrap().as_str().parse::<u16>().unwrap() + 1;
            self.new_pattern.replace("%d", &number.to_string())

        }
    }
}
