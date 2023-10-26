mod docker;
mod change_parser;

use clap::{Args, Subcommand};

use crate::subcommands::describe::change_parser::ChangeTriggerParser;

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
        match &self.subcommand {
            Some(c) => match c {
                DescribeSubCommands::Docker(cc) => {cc.describe_docker();},
            },
            None => self.base_action(),
        }
    }

    fn base_action(&self) {
        println!("Basic describe called");
        let trigger = ChangeTriggerParser::parse("(type = test AND breaking)ORtype=testORscope=core-depsANDtype=featANDtypeIN[test,feat,fix]");
        dbg!(trigger);
    }
}

#[derive(Subcommand, Clone)]
#[derive(Debug)]
enum DescribeSubCommands {
    #[command(about = "TODO")]
    Docker(DescribeDockerSubCommand),
}
