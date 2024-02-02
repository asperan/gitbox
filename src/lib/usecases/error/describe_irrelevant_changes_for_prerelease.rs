use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct DescribeNoRelevantChangesError {}

impl DescribeNoRelevantChangesError {
    pub fn new() -> DescribeNoRelevantChangesError {
        DescribeNoRelevantChangesError {}
    }
}

impl Display for DescribeNoRelevantChangesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There are no relevant changes from the last release. Use triggers if you want to proc a new version.")
    }
}

impl Error for DescribeNoRelevantChangesError {}
