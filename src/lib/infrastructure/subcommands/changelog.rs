use std::rc::Rc;

use clap::Args;

use crate::{
    application::{
        controller::{changelog::ChangelogController, exit_code::ControllerExitCode},
        manager::output_manager::OutputManager,
        options::changelog::ChangelogOptions,
        retriever::gitinfo_retriever::GitInfoIngressManager,
    },
    infrastructure::{
        git_cli::GitCli, output_manager_impl::OutputManagerImpl, subcommand::Subcommand,
    },
};

#[derive(Args, Debug)]
#[command(about = "Generate a changelog")]
pub struct ChangelogSubCommand {
    #[arg(
        long,
        help = "If set, the changelog will be generated with changes since the last version rather than the last stable release"
    )]
    from_latest_version: bool,
    #[arg(
        short = 'T',
        long,
        help = "Set the title format. The content placeholder is '%s'",
        default_value("# %s"),
        allow_hyphen_values(true)
    )]
    title_format: String,
    #[arg(
        short = 't',
        long,
        help = "Set the type format. The content placeholder is '%s'",
        default_value("= %s"),
        allow_hyphen_values(true)
    )]
    type_format: String,
    #[arg(
        short = 's',
        long,
        help = "Set the scope format. The content placeholder is '%s'",
        default_value("- %s"),
        allow_hyphen_values(true)
    )]
    scope_format: String,
    #[arg(
        short = 'l',
        long,
        help = "Set the list format. The content placeholder is '%s'",
        default_value("%s"),
        allow_hyphen_values(true)
    )]
    list_format: String,
    #[arg(
        short = 'i',
        long,
        help = "Set the list item format. The content placeholder is '%s'",
        default_value("* %s"),
        allow_hyphen_values(true)
    )]
    item_format: String,
    #[arg(
        short = 'b',
        long,
        help = "Set the breaking commit format. The content placeholder is '%s'",
        default_value("!!! %s "),
        allow_hyphen_values(true)
    )]
    breaking_format: String,

    #[arg(
        long,
        help = "Set the trigger to use to exclude commits from the changelog. For more informations about the grammar, run 'help grammar'"
    )]
    exclude_trigger: Option<String>,
}

impl Subcommand for ChangelogSubCommand {
    fn execute(&self) -> i32 {
        let output_manager = Rc::new(OutputManagerImpl::new());
        let git_cli = Rc::new(GitCli::new());
        if let Err(e) = git_cli.git_dir() {
            output_manager.error(&format!("Failed to retrieve git dir: {}", e.to_string()));
            output_manager.error("changelog subcommand cannot be called outside of a git dir");
            return 1;
        }
        match ChangelogOptions::new(
            self.from_latest_version,
            self.title_format.clone(),
            self.type_format.clone(),
            self.scope_format.clone(),
            self.list_format.clone(),
            self.item_format.clone(),
            self.breaking_format.clone(),
            self.exclude_trigger.clone(),
        ) {
            Ok(options) => {
                let controller = ChangelogController::new(
                    options,
                    git_cli.clone(),
                    git_cli.clone(),
                    output_manager.clone(),
                );
                match controller.changelog() {
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
