mod application;
mod domain;
mod infrastructure;
mod usecase;

use std::process::exit;

use crate::infrastructure::{subcommand::Subcommand, subcommands::Commands};
use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[command(name = "gb")]
#[command(author = "Alex Speranza")]
#[command(version = "1.1.0")]
#[command(about = "Gitbox (gb) is wrapper for git and it enhance some functionalities.", long_about = None)]
struct CliParser {
    #[command(subcommand)]
    command: Commands,
}

pub fn run() {
    let cli = CliParser::parse();

    #[cfg(debug_assertions)]
    dbg!(&cli.command);

    exit(match &cli.command {
        Commands::Changelog(c) => c.execute(),
        Commands::Init(c) => c.execute(),
        Commands::Complete(c) => {
            c.print_completion_script(&mut CliParser::command_for_update());
            0
        }
        Commands::Commit(c) => c.execute(),
        Commands::Describe(c) => c.execute(),
        Commands::RefreshExtra(c) => c.execute(),
        Commands::License(c) => c.execute(),
        Commands::Tree(c) => c.execute(),
        // Catch-all branch for hidden commands
        _ => {
            eprintln!("Unknown command. See '--help' or subcommand 'help' for available commands");
            1
        }
    });
}
