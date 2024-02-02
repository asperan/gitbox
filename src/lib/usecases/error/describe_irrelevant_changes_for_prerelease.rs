use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DescribeIrrelevantChangesForPrereleaseError {}

impl DescribeIrrelevantChangesForPrereleaseError {
    pub fn new() -> DescribeIrrelevantChangesForPrereleaseError {
        DescribeIrrelevantChangesForPrereleaseError {}
    }
}

impl Display for DescribeIrrelevantChangesForPrereleaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There are no relevant changes from the last release. Use triggers if you want to proc a new version.")
    }
}

impl Error for DescribeIrrelevantChangesForPrereleaseError {}
