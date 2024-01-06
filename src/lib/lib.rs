mod common;
mod subcommands;

use crate::{common::commons::print_error_and_exit, subcommands::Commands};
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

    match &cli.command {
        Commands::Changelog(c) => c.changelog(),
        Commands::Commit(c) => c.commit(),
        Commands::Complete(c) => c.print_completion_script(&mut CliParser::command_for_update()),
        Commands::Describe(c) => c.describe(),
        Commands::Init(c) => c.init_repository(),
        Commands::License(c) => c.create_license(),
        Commands::Tree(c) => c.print_tree(),
        // Catch-all branch for hidden commands
        _ => print_error_and_exit(
            "Unknown command. See '--help' or subcommand 'help' for available commands",
        ),
    }
}
