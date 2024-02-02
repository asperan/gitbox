use crate::{domain::trigger::Trigger, usecases::metadata_spec::MetadataSpec};

pub type PrereleasePattern = Box<dyn Fn(u32) -> String>;
pub type OldPrereleasePattern = Box<dyn Fn(&String) -> u32>;

pub struct DescribeConfiguration {
    prerelease: bool,
    prerelease_pattern: PrereleasePattern,
    old_prerelease_pattern: OldPrereleasePattern,
    prerelease_pattern_changed: bool,
    metadata: Vec<MetadataSpec>,
    major_trigger: Trigger,
    minor_trigger: Trigger,
    patch_trigger: Trigger,
}

impl DescribeConfiguration {
    pub fn new(
        prerelease: bool,
        prerelease_pattern: PrereleasePattern,
        old_prerelease_pattern: OldPrereleasePattern,
        prerelease_pattern_changed: bool,
        metadata: Vec<MetadataSpec>,
        major_trigger: Trigger,
        minor_trigger: Trigger,
        patch_trigger: Trigger,
    ) -> DescribeConfiguration {
        DescribeConfiguration {
            prerelease,
            prerelease_pattern,
            old_prerelease_pattern,
            prerelease_pattern_changed,
            metadata,
            major_trigger,
            minor_trigger,
            patch_trigger,
        }
    }

    pub fn prerelease(&self) -> bool {
        self.prerelease
    }
    pub fn prerelease_pattern(&self) -> &PrereleasePattern {
        &self.prerelease_pattern
    }
    pub fn old_prerelease_pattern(&self) -> &OldPrereleasePattern {
        &self.old_prerelease_pattern
    }

    pub fn prerelease_pattern_changed(&self) -> bool {
        self.prerelease_pattern_changed
    }

    pub fn metadata(&self) -> &Vec<MetadataSpec> {
        &self.metadata
    }
    pub fn major_trigger(&self) -> &Trigger {
        &self.major_trigger
    }
    pub fn minor_trigger(&self) -> &Trigger {
        &self.minor_trigger
    }
    pub fn patch_trigger(&self) -> &Trigger {
        &self.patch_trigger
    }
}
