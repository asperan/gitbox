use crate::{domain::trigger::Trigger, usecases::metadata_spec::MetadataSpec};

pub type PrereleasePattern<'a> = Box<dyn Fn(u32) -> String + 'a>;
pub type OldPrereleasePattern<'a> = Box<dyn Fn(&String) -> u32 + 'a>;

pub struct DescribeConfiguration<'a> {
    prerelease: bool,
    prerelease_pattern: PrereleasePattern<'a>,
    old_prerelease_pattern: OldPrereleasePattern<'a>,
    prerelease_pattern_changed: bool,
    metadata: Vec<MetadataSpec>,
    major_trigger: Trigger,
    minor_trigger: Trigger,
    patch_trigger: Trigger,
}

impl<'a> DescribeConfiguration<'a> {
    pub fn new(
        prerelease: bool,
        prerelease_pattern: PrereleasePattern<'a>,
        old_prerelease_pattern: OldPrereleasePattern<'a>,
        prerelease_pattern_changed: bool,
        metadata: Vec<MetadataSpec>,
        major_trigger: Trigger,
        minor_trigger: Trigger,
        patch_trigger: Trigger,
    ) -> DescribeConfiguration<'a> {
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
