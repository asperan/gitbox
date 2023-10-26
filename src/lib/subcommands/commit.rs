mod prompt;
mod refresh;

use clap::{Args, Subcommand};

use crate::common::{command_issuer::CommandIssuer, commons::print_cli_error_message_and_exit};

use self::{prompt::Prompt, refresh::RefreshTypesAndScopesSubcommand};

#[derive(Args, Debug)]
#[command(about = "Create a commit with a conventional message")]
pub struct CommitSubCommand {
    #[arg(short = 't', long = "type", help = "Set the type of the commit")]
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

    #[arg(short = 's', long, help = "Set the summary of the commit")]
    summary: Option<String>,

    #[arg(short = 'm', long, help = "Set the body of the commit")]
    message: Option<String>,

    #[arg(short, long, help = "Suppress the print of the complete message")]
    quiet: bool,

    #[command(subcommand)]
    subcommand: Option<CommitSubCommands>,
}

#[derive(Subcommand, Clone, Debug)]
enum CommitSubCommands {
    #[command(about = "Refresh the lists of already used types and scopes (best result is after a fetch)")]
    Refresh(RefreshTypesAndScopesSubcommand),
}

impl CommitSubCommand {
    pub fn commit(&self) {
        match &self.subcommand {
            Some(c) => match c {
                CommitSubCommands::Refresh(refresh) => refresh.refresh_types_and_scopes(),
            },
            None => {
                let full_message = &self.full_commit_message();
                if !self.quiet {
                    println!("{}", full_message);
                }
                let result = CommandIssuer::git(vec!["commit", "-m", full_message]);
                if !result.status.success() {
                    print_cli_error_message_and_exit(&result.stderr, "create a commit");
                }
            }
        }
    }

    fn full_commit_message(&self) -> String {
        let commit_type = self.commit_type();
        let scope = self.scope();
        let breaking = if self.breaking() { "!" } else { "" };
        let summary = self.summary();
        let body = self.body().trim().to_string();

        format!(
            "{}{}{}: {}{}",
            commit_type,
            if scope.is_empty() {
                "".to_string()
            } else {
                format!("({})", scope)
            },
            breaking,
            summary,
            if body.is_empty() {
                "".to_string()
            } else {
                format!("\n\n{}", body)
            }
        )
    }

    fn commit_type(&self) -> String {
        self.commit_type.clone().unwrap_or_else(Prompt::ask_type)
    }

    fn scope(&self) -> String {
        self.scope.clone().unwrap_or_else(Prompt::ask_scope)
    }

    fn breaking(&self) -> bool {
        if !self.is_breaking && !self.is_not_breaking {
            Prompt::ask_breaking()
        } else {
            self.is_breaking
        }
    }

    fn summary(&self) -> String {
        self.summary.clone().unwrap_or_else(Prompt::ask_summary)
    }

    fn body(&self) -> String {
        self.message.clone().unwrap_or_else(Prompt::ask_body)
    }
}

