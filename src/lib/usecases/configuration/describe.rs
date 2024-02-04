use crate::{domain::trigger::Trigger, usecases::metadata_spec::MetadataSpec};

pub type PrereleasePattern<'a> = Box<dyn Fn(u32) -> String + 'a>;
pub type OldPrereleasePattern<'a> = Box<dyn Fn(&String) -> u32 + 'a>;

pub struct DescribeConfiguration<'a> {
    prerelease: DescribePrereleaseConfiguration<'a>,
    metadata: DescribeMetadataConfiguration,
    triggers: DescribeTriggerConfiguration,
}

impl<'a> DescribeConfiguration<'a> {
    pub fn new(
        prerelease: DescribePrereleaseConfiguration<'a>,
        metadata: DescribeMetadataConfiguration,
        triggers: DescribeTriggerConfiguration,
    ) -> DescribeConfiguration<'a> {
        DescribeConfiguration {
            prerelease,
            metadata,
            triggers,
        }
    }
    pub fn prerelease(&self) -> &DescribePrereleaseConfiguration {
        &self.prerelease
    }
    pub fn metadata(&self) -> &DescribeMetadataConfiguration {
        &self.metadata
    }
    pub fn triggers(&self) -> &DescribeTriggerConfiguration {
        &self.triggers
    }
}

pub struct DescribePrereleaseConfiguration<'a> {
    prerelease: bool,
    prerelease_pattern: PrereleasePattern<'a>,
    old_prerelease_pattern: OldPrereleasePattern<'a>,
    prerelease_pattern_changed: bool,
}

impl<'a> DescribePrereleaseConfiguration<'a> {
    pub fn new(
        prerelease: bool,
        prerelease_pattern: PrereleasePattern<'a>,
        old_prerelease_pattern: OldPrereleasePattern<'a>,
        prerelease_pattern_changed: bool,
    ) -> DescribePrereleaseConfiguration<'a> {
        DescribePrereleaseConfiguration {
            prerelease,
            prerelease_pattern,
            old_prerelease_pattern,
            prerelease_pattern_changed,
        }
    }

    pub fn prerelease(&self) -> bool {
        self.prerelease
    }
    pub fn prerelease_pattern(&self) -> &PrereleasePattern<'a> {
        &self.prerelease_pattern
    }
    pub fn old_prerelease_pattern(&self) -> &OldPrereleasePattern<'a> {
        &self.old_prerelease_pattern
    }
    pub fn prerelease_pattern_changed(&self) -> bool {
        self.prerelease_pattern_changed
    }
}

pub struct DescribeMetadataConfiguration {
    specs: Vec<MetadataSpec>,
}

impl DescribeMetadataConfiguration {
    pub fn new(specs: Vec<MetadataSpec>) -> DescribeMetadataConfiguration {
        DescribeMetadataConfiguration { specs }
    }

    pub fn specs(&self) -> &Vec<MetadataSpec> {
        &self.specs
    }
}

pub struct DescribeTriggerConfiguration {
    major_trigger: Trigger,
    minor_trigger: Trigger,
    patch_trigger: Trigger,
}

impl DescribeTriggerConfiguration {
    pub fn new(
        major_trigger: Trigger,
        minor_trigger: Trigger,
        patch_trigger: Trigger,
    ) -> DescribeTriggerConfiguration {
        DescribeTriggerConfiguration {
            major_trigger,
            minor_trigger,
            patch_trigger,
        }
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
