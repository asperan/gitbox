use std::rc::Rc;

use clap::{builder::PossibleValue, Args, ValueEnum};

use crate::{
    application::{
        controller::{describe::DescribeController, exit_code::ControllerExitCode},
        manager::{
            gitinfo_ingress_manager::GitInfoIngressManager,
            message_egress_manager::MessageEgressManager,
        },
        options::describe::DescribeOptions,
    },
    infrastructure::{
        git_cli::GitCli, output_manager_impl::MessageEgressManagerImpl, subcommand::Subcommand,
    },
    usecases::metadata_spec::MetadataSpec,
};

#[derive(Args, Debug)]
#[command(about = "Calculate the next version")]
pub struct DescribeSubCommand {
    #[arg(long, help = "Describe a prerelease")]
    prerelease: bool,

    #[arg(
        long,
        help = "Set the pattern for the new prerelease. A number can be used with the placeholder '%d'",
        requires("prerelease"),
        default_value = "dev%d"
    )]
    prerelease_pattern: String,
    #[arg(
        long,
        help = "Set the pattern of the old prerelease. Uses the same placeholder as '--prerelease-pattern'. Use this option when changing prerelease pattern. Defaults to the prerelease pattern",
        requires("prerelease")
    )]
    old_prerelease_pattern: Option<String>,

    #[arg(
        short,
        long,
        help = "Print the last version (if possible) in addition to the new version"
    )]
    diff: bool,

    #[arg(short, long, help = "Add a metadata to include in the new version (can be used multiple times)", value_parser = clap::builder::EnumValueParser::<MetadataSpec>::new())]
    metadata: Vec<MetadataSpec>,

    #[arg(
        long,
        help = "Set the expression which triggers a major change (Default behaviour is equivalent to 'breaking'). For more informations about the grammar, run 'help grammar'"
    )]
    major_trigger: Option<String>,
    #[arg(
        long,
        help = "Set the expression which triggers a minor change (Default behaviour is equivalent to 'type IN [ feat ]'). For more informations about the grammar, run 'help grammar'"
    )]
    minor_trigger: Option<String>,
    #[arg(
        long,
        help = "Set the expression which triggers a patch change (Default behaviou is equivalent to 'type IN [ fix ]'). For more informations about the grammar, run 'help grammar'"
    )]
    patch_trigger: Option<String>,

    #[arg(short = 't', long, help = "Create a new tag with the computed version")]
    create_tag: bool,
    #[arg(
        short = 'M',
        long,
        help = "Set the additional message for the created tag",
        requires("create_tag"),
        value_parser = clap::builder::NonEmptyStringValueParser::new()
    )]
    tag_message: Option<String>,
    #[arg(
        short = 's',
        long,
        help = "If set, the created tag is signed",
        requires("create_tag")
    )]
    sign_tag: bool,
}

impl Subcommand for DescribeSubCommand {
    fn execute(&self) -> i32 {
        let git_cli = Rc::new(GitCli::new());
        let output_manager = Rc::new(MessageEgressManagerImpl::new());
        if let Err(e) = git_cli.git_dir() {
            output_manager.error(&format!("Failed to retrieve git dir: {}", e.to_string()));
            output_manager.error("describe subcommand can only be run inside a git project");
            return 1;
        }
        match DescribeOptions::new(
            self.prerelease,
            self.prerelease_pattern.clone(),
            self.old_prerelease_pattern
                .clone()
                .unwrap_or(self.prerelease_pattern.clone()),
            self.diff,
            self.metadata.clone(),
            self.major_trigger.clone(),
            self.minor_trigger.clone(),
            self.patch_trigger.clone(),
            self.create_tag,
            self.tag_message.clone(),
            self.sign_tag,
        ) {
            Ok(options) => {
                let controller = DescribeController::new(
                    options,
                    git_cli.clone(),
                    git_cli.clone(),
                    git_cli.clone(),
                    git_cli.clone(),
                    output_manager.clone(),
                );
                match controller.describe() {
                    ControllerExitCode::Ok => 0,
                    ControllerExitCode::Error(i) => i,
                }
            }
            Err(e) => {
                output_manager.error(&e.to_string());
                1
            }
        }
    }
}

impl ValueEnum for MetadataSpec {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Sha, Self::Date]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Sha => Some(PossibleValue::new("sha")),
            Self::Date => Some(PossibleValue::new("date")),
        }
    }
}
