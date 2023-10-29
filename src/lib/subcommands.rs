use clap::Subcommand;

use self::{
    changelog::ChangelogSubCommand, commit::CommitSubCommand, complete::CompleteSubCommand,
    describe::DescribeSubCommand, grammar::GrammarSubCommand, init::InitSubCommand,
    license::LicenseSubCommand, tree::TreeSubCommand,
};

mod changelog;
mod commit;
mod complete;
mod describe;
mod grammar;
mod init;
mod license;
mod tree;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Changelog(ChangelogSubCommand),
    Commit(CommitSubCommand),
    Complete(CompleteSubCommand),
    Describe(DescribeSubCommand),
    Init(InitSubCommand),
    License(LicenseSubCommand),
    Tree(TreeSubCommand),
    // HIDDEN COMMANDS
    Grammar(GrammarSubCommand),
}
