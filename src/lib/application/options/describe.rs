use crate::usecases::metadata_spec::MetadataSpec;

pub struct DescribeOptions {
    prerelease: bool,
    prerelease_pattern: String,
    old_prerelease_pattern: String,
    diff: bool,
    metadata: Vec<MetadataSpec>,
    major_trigger: Option<String>,
    minor_trigger: Option<String>,
    patch_trigger: Option<String>,
    create_tag: bool,
    tag_message: Option<String>,
    sign_tag: bool,
}

impl DescribeOptions {
    pub fn new(
        prerelease: bool,
        prerelease_pattern: String,
        old_prerelease_pattern: String,
        diff: bool,
        metadata: Vec<MetadataSpec>,
        major_trigger: Option<String>,
        minor_trigger: Option<String>,
        patch_trigger: Option<String>,
        create_tag: bool,
        tag_message: Option<String>,
        sign_tag: bool,
    ) -> DescribeOptions {
        DescribeOptions {
            prerelease,
            prerelease_pattern,
            old_prerelease_pattern,
            diff,
            metadata,
            major_trigger,
            minor_trigger,
            patch_trigger,
            create_tag,
            tag_message,
            sign_tag,
        }
    }

    pub fn prerelease(&self) -> bool {
        self.prerelease
    }
    pub fn prerelease_pattern(&self) -> &str {
        &self.prerelease_pattern
    }
    pub fn old_prerelease_pattern(&self) -> &str {
        &self.old_prerelease_pattern
    }
    pub fn diff(&self) -> bool {
        self.diff
    }
    pub fn metadata(&self) -> &Vec<MetadataSpec> {
        &self.metadata
    }
    pub fn major_trigger(&self) -> &Option<String> {
        &self.major_trigger
    }
    pub fn minor_trigger(&self) -> &Option<String> {
        &self.minor_trigger
    }
    pub fn patch_trigger(&self) -> &Option<String> {
        &self.patch_trigger
    }
    pub fn create_tag(&self) -> bool {
        self.create_tag
    }
    pub fn tag_message(&self) -> &Option<String> {
        &self.tag_message
    }
    pub fn sign_tag(&self) -> bool {
        self.sign_tag
    }
}
