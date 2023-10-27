mod docker;
mod change_parser;
mod stable;
mod metadata;
mod prerelease;

use clap::{Args, Subcommand};

use crate::{subcommands::describe::{stable::StableVersionCalculator, metadata::MetadataGenerator, prerelease::PrereleaseUpdater}, common::{cached_values::CachedValues, semantic_version::SemanticVersion, commons::print_error_and_exit}};

use self::{docker::DescribeDockerSubCommand, metadata::MetadataSpecs};

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Calculate the next version")]
pub struct DescribeSubCommand {
    #[command(subcommand)]
    subcommand: Option<DescribeSubCommands>,

    #[arg(long, help = "Describe a prerelease")]
    prerelease: bool,

    #[arg(long, help = "Set the pattern for the new prerelease. A number can be used with the placeholder '%d'", requires("prerelease"), default_value = "dev%d")]
    prerelease_pattern: String,
    #[arg(long, help = "Set the pattern of the old prerelease. Uses the same placeholder as '--prerelease-pattern'. Use this option when changing prerelease pattern", requires("prerelease"), default_value = "dev%d")]
    old_prerelease_pattern: String,

    #[arg(short, long, help = "Print the last version (if possible) in addition to the new version")]
    diff: bool,

    #[arg(short, long, help = "Add a metadata to include in the new version (can be used multiple times)", value_parser = clap::builder::EnumValueParser::<MetadataSpecs>::new())]
    metadata: Vec<MetadataSpecs>,

    #[arg(long, help = "Set the expression which triggers a major change")]
    major_trigger: Option<String>,
    #[arg(long, help = "Set the expression which triggers a minor change")]
    minor_trigger: Option<String>,
    #[arg(long, help = "Set the expression which triggers a patch change")]
    patch_trigger: Option<String>,

    #[arg(short = 't', long, help = "Create a new signed tag with the computed version")]
    create_tag: bool,
    #[arg(short = 'M', long, help = "Set the additional message for the created tag", requires("create_tag"))]
    tag_message: Option<String>,
}

impl DescribeSubCommand {
    pub fn describe(&self) {
        println!("describe called");
        let new_version = self.update_version();
        match &self.subcommand {
            Some(c) => match c {
                DescribeSubCommands::Docker(cc) => {cc.describe_docker();},
            },
            None => self.print_version(&new_version, CachedValues::last_version().as_ref()),
        }
    }

    fn update_version(&self) -> SemanticVersion {
        let stable_updater = StableVersionCalculator::new(&self.major_trigger, &self.minor_trigger, &self.patch_trigger);
        let mut new_version = stable_updater.next_stable(CachedValues::last_stable_release());
        if CachedValues::last_stable_release().as_ref().is_some_and(|v| new_version == *v) {
            print_error_and_exit("There are no relevant changes since the last stable version. Change triggers or commit some relevant changes to describe a new version.")
        }

        let prerelease = if self.prerelease {
            let prerelease_updater = PrereleaseUpdater::new(&self.prerelease_pattern, &self.old_prerelease_pattern);
            Some(prerelease_updater.update_prerelease(&new_version, CachedValues::last_version()))
        } else {
            None
        };
        let metadata_str = MetadataGenerator::generate(&self.metadata);
        new_version.add_prerelease(prerelease);
        new_version.add_metadata(metadata_str);
        new_version
    }

    fn print_version(&self, new_version: &SemanticVersion, old_version: Option<&SemanticVersion>) {
        let left_part = if self.diff {
            format!("{} -> ", old_version.map_or(String::from("none"), |v| v.to_string()))
        } else {
            String::from("")
        };
        println!("{}{}", left_part, new_version);
    }
}

#[derive(Subcommand, Clone)]
#[derive(Debug)]
enum DescribeSubCommands {
    #[command(about = "TODO")]
    Docker(DescribeDockerSubCommand),
}

