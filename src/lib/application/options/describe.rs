use crate::{
    application::error::describe_options_invariant_error::DescribeOptionsInvariantError,
    usecase::{metadata_spec::MetadataSpec, type_aliases::AnyError},
};

pub const PRERELEASE_NUM_PLACEHOLDER: &str = "%d";

pub struct DescribeOptions {
    prerelease_options: DescribePrereleaseOptions,
    diff: bool,
    metadata_options: DescribeMetadataOptions,
    trigger_options: DescribeTriggerOptions,
    tag_options: DescribeTagOptions,
}

impl DescribeOptions {
    pub fn new(
        prerelease_options: DescribePrereleaseOptions,
        diff: bool,
        metadata_options: DescribeMetadataOptions,
        trigger_options: DescribeTriggerOptions,
        tag_options: DescribeTagOptions,
    ) -> DescribeOptions {
        DescribeOptions {
            prerelease_options,
            diff,
            metadata_options,
            trigger_options,
            tag_options,
        }
    }

    pub fn prerelease(&self) -> &DescribePrereleaseOptions {
        &self.prerelease_options
    }

    pub fn diff(&self) -> bool {
        self.diff
    }
    pub fn metadata(&self) -> &DescribeMetadataOptions {
        &self.metadata_options
    }
    pub fn triggers(&self) -> &DescribeTriggerOptions {
        &self.trigger_options
    }
    pub fn tag(&self) -> &DescribeTagOptions {
        &self.tag_options
    }
}

#[derive(Debug)]
pub struct DescribePrereleaseOptions {
    enabled: bool,
    pattern: String,
    old_pattern: String,
}

impl DescribePrereleaseOptions {
    pub fn new(enabled: bool, pattern: String, old_pattern: String) -> Result<Self, AnyError> {
        Self::check_pattern_has_placeholder(&pattern, "prerelease pattern")?;
        Self::check_pattern_has_placeholder(&old_pattern, "old prerelease patter")?;

        Ok(DescribePrereleaseOptions {
            enabled,
            pattern,
            old_pattern,
        })
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    pub fn old_pattern(&self) -> &str {
        &self.old_pattern
    }

    fn check_pattern_has_placeholder(
        pattern: &str,
        what: &str,
    ) -> Result<(), DescribeOptionsInvariantError> {
        if pattern.contains(PRERELEASE_NUM_PLACEHOLDER) {
            Ok(())
        } else {
            Err(DescribeOptionsInvariantError::new(&format!(
                "{} '{}' did not contain '{}' placeholder",
                what, pattern, PRERELEASE_NUM_PLACEHOLDER
            )))
        }
    }
}

#[derive(Debug)]
pub struct DescribeMetadataOptions {
    metadata: Vec<MetadataSpec>,
}

impl DescribeMetadataOptions {
    pub fn new(metadata: Vec<MetadataSpec>) -> Self {
        DescribeMetadataOptions { metadata }
    }

    pub fn specs(&self) -> &[MetadataSpec] {
        self.metadata.as_slice()
    }
}

#[derive(Debug)]
pub struct DescribeTriggerOptions {
    major_trigger: Option<String>,
    minor_trigger: Option<String>,
    patch_trigger: Option<String>,
}

impl DescribeTriggerOptions {
    pub fn new(
        major_trigger: Option<String>,
        minor_trigger: Option<String>,
        patch_trigger: Option<String>,
    ) -> Self {
        DescribeTriggerOptions {
            major_trigger,
            minor_trigger,
            patch_trigger,
        }
    }
    pub fn major(&self) -> Option<&str> {
        self.major_trigger.as_deref()
    }
    pub fn minor(&self) -> Option<&str> {
        self.minor_trigger.as_deref()
    }
    pub fn patch(&self) -> Option<&str> {
        self.patch_trigger.as_deref()
    }
}

#[derive(Debug)]
pub struct DescribeTagOptions {
    create_tag: bool,
    tag_message: Option<String>,
    sign_tag: bool,
}

impl DescribeTagOptions {
    pub fn new(create_tag: bool, tag_message: Option<String>, sign_tag: bool) -> Self {
        DescribeTagOptions {
            create_tag,
            tag_message,
            sign_tag,
        }
    }
    pub fn enabled(&self) -> bool {
        self.create_tag
    }
    pub fn message(&self) -> Option<&str> {
        self.tag_message.as_deref()
    }
    pub fn sign_enabled(&self) -> bool {
        self.sign_tag
    }
}
