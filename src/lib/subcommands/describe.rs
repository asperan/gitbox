mod docker;
mod change_parser;
mod stable;

use clap::{Args, Subcommand};

use crate::{subcommands::describe::stable::StableVersionCalculator, common::cached_values::CachedValues};

use self::docker::DescribeDockerSubCommand;

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Calculate the next version")]
pub struct DescribeSubCommand {
    #[command(subcommand)]
    subcommand: Option<DescribeSubCommands>,

    #[arg(long, help = "Describe a prerelease")]
    prerelease: bool,

    #[arg(long, help = "Set the pattern for the new prerelease. A number can be used with the placeholder '%d'", requires("prerelease"))]
    prerelease_pattern: Option<String>,
    #[arg(long, help = "Set the pattern of the old prerelease. Uses the same placeholder as '--prerelease-pattern'. Use this option when changing prerelease pattern", requires("prerelease"))]
    old_prerelease_pattern: Option<String>,

    #[arg(short, long, help = "Print the last version (if possible) in addition to the new version")]
    diff: bool,

    #[arg(short, long, help = "Add a metadata to include in the new version (can be used multiple times)")]
    metadata: Vec<String>,

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
            None => self.print_version(&new_version, CachedValues::last_version().as_deref()),
        }
    }

    fn update_version(&self) -> String {
        /*
        println!("Basic describe called");
        let test_trigger = "scope IN [core-deps, frontend] AND type IN [ test, feat ] OR breaking";
        // let test_trigger = "type IN [ test, feat ]";
        // let test_trigger = "breaking";
        // let test_trigger = "scope IN [ core-deps, backend ]";
        let trigger = ChangeTriggerParser::run(test_trigger);
        dbg!(&trigger);
        dbg!(trigger.call("test", &Some("backend".to_string()), false));
        */
        dbg!(CachedValues::last_version());
        dbg!(CachedValues::last_release());
        let stable_updater = StableVersionCalculator::new(&self.major_trigger, &self.minor_trigger, &self.patch_trigger);
        let new_stable_version = stable_updater.next_stable(CachedValues::last_release());
        // TODO: calc prerelease (needs last_version and new_stable_version), returns a Option? and
        // starts as None?
        // TODO: calc metadata, it is an Option
        // TODO: final version: format!("{}{}{}", new_stable_version, prerelease.map_or(|p|
        // format!("-{}", p), ""), metadata.map_or(|m| format!("+{}", m), "")
        // TODO: if new_version == last_release.stable_version && !prerelease => error "No
        // important changes since last release. Change triggers or commit some relevant changes."
        dbg!(&new_stable_version);
        new_stable_version
    }

    fn print_version(&self, new_version: &str, old_version: Option<&str>) {
        let left_part = if self.diff {
            format!("{} -> ", old_version.unwrap_or("none"))
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

