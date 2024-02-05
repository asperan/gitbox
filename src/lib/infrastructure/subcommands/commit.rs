use std::rc::Rc;

use clap::{builder::NonEmptyStringValueParser, Args, Subcommand as ClapSubcommand};

use crate::{
    application::{
        controller::{commit::CommitController, exit_code::ControllerExitCode},
        manager::message_egress_manager::MessageEgressManager,
        options::commit::CommitOptions,
    },
    infrastructure::{
        git_cli::GitCli, gitextra_manager_impl::GitExtraManagerImpl,
        output_manager_impl::OutputManagerImpl, prompt_manager::PromptManager,
        subcommand::Subcommand, subcommands::commit_refresh::RefreshTypesAndScopesSubcommand,
    },
    usecases::type_aliases::AnyError,
};

#[derive(Args, Debug)]
#[command(about = "Create a commit with a conventional message")]
pub struct CommitSubCommand {
    #[arg(short = 't', long = "type", help = "Set the type of the commit", value_parser = NonEmptyStringValueParser::new())]
    commit_type: Option<String>,

    #[arg(short = 'S', long, help = "Set the scope of the commit")]
    scope: Option<String>,

    #[arg(
        long = "breaking",
        help = "Flag the commit as a breaking change",
        overrides_with = "is_not_breaking"
    )]
    is_breaking: bool,
    #[arg(
        long = "no-breaking",
        help = "Flag the commit as not a breaking change",
        overrides_with = "is_breaking"
    )]
    is_not_breaking: bool,

    #[arg(short = 's', long, help = "Set the summary of the commit", value_parser = NonEmptyStringValueParser::new())]
    summary: Option<String>,

    #[arg(short = 'm', long, help = "Set the body of the commit")]
    message: Option<String>,

    #[arg(short, long, help = "Suppress the print of the complete message")]
    quiet: bool,

    #[command(subcommand)]
    subcommand: Option<CommitSubCommands>,
}

#[derive(ClapSubcommand, Clone, Debug)]
enum CommitSubCommands {
    #[command(
        about = "Refresh the lists of already used types and scopes (best result is after a fetch)"
    )]
    Refresh(RefreshTypesAndScopesSubcommand),
}

impl Subcommand for CommitSubCommand {
    fn execute(&self) -> i32 {
        match &self.subcommand {
            Some(c) => match c {
                CommitSubCommands::Refresh(refresh) => refresh.execute(),
            },
            None => self.basic_command(),
        }
    }
}

impl CommitSubCommand {
    fn basic_command(&self) -> i32 {
        let git_cli = Rc::new(GitCli::new());
        let output_manager = Rc::new(OutputManagerImpl::new());
        let gitextra_manager = Rc::new(GitExtraManagerImpl::new(git_cli.clone()));
        let prompt_manager = PromptManager::new(gitextra_manager.clone(), gitextra_manager.clone());
        let options = match self.ask_missing_fields(prompt_manager) {
            Ok(o) => o,
            Err(e) => {
                output_manager.error(&e.to_string());
                return 1;
            }
        };
        let commit_manager = git_cli.clone();
        let controller = CommitController::new(options, commit_manager, output_manager);
        match controller.commit() {
            ControllerExitCode::Ok => 0,
            ControllerExitCode::Error(i) => i,
        }
    }

    fn breaking_option(&self) -> Option<bool> {
        match (self.is_breaking, self.is_not_breaking) {
            (false, false) => None,
            (false, true) => Some(false),
            (true, false) => Some(true),
            (true, true) => unreachable!(),
        }
    }

    fn ask_missing_fields(&self, prompt_manager: PromptManager) -> Result<CommitOptions, AnyError> {
        let temp_type = match self.commit_type.clone() {
            Some(t) => t,
            None => prompt_manager.ask_type()?,
        };
        let temp_scope = match self.scope.clone() {
            Some(s) if !s.is_empty() => Some(s),
            Some(_) => None,
            None => prompt_manager.ask_scope()?,
        };
        let temp_breaking = match self.breaking_option() {
            Some(b) => b,
            None => prompt_manager.ask_breaking()?,
        };
        let temp_summary = match self.summary.clone() {
            Some(s) => s,
            None => prompt_manager.ask_summary()?,
        };
        let temp_message = match self.message.clone() {
            Some(s) if !s.is_empty() => Some(s),
            Some(_) => None,
            None => prompt_manager.ask_body()?,
        };
        CommitOptions::new(
            temp_type,
            temp_scope,
            temp_breaking,
            temp_summary,
            temp_message,
            self.quiet,
        )
    }
}
