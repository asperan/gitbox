mod subcommands;
mod common;

use clap::Parser;
use crate::subcommands::Commands;

#[derive(Parser,Debug)]
#[command(name = "gitbox")]
#[command(author = "Alex Speranza")]
#[command(version = "0.1.0-dev1")]
#[command(about = "Enhance git", long_about = None)]
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
        Commands::Complete(c) => c.print_completion_script(),
        Commands::Describe(c) => c.describe(),
        Commands::Init(c) => c.init_repository(),
        Commands::License(c) => c.create_license(),
        Commands::Tree(c) => c.print_tree(),
    }
}

