use clap::Args;

use crate::common::semantic_version::SemanticVersion;

#[derive(Args, Clone, Debug)]
pub struct DescribeDockerSubCommand {
    #[arg(
        short,
        long,
        help = "The separator to use between versions",
        default_value = "\n"
    )]
    separator: String,
    #[arg(long, help = "Do not print 'latest' as first version")]
    exclude_latest: bool,
}

impl DescribeDockerSubCommand {
    pub fn describe_docker(&self, version: &SemanticVersion) {
        let mut versions: Vec<String> = vec![];
        if !self.exclude_latest {
            versions.push(String::from("latest"));
        }
        versions.push(version.to_string().split('+').next().unwrap().to_string());
        if version.prerelease().is_none() {
            versions.push(format!("{}.{}", version.major(), version.minor()));
            versions.push(format!("{}", version.major()));
        }
        println!("{}", versions.join(&self.separator));
    }
}
