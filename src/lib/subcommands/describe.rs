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
    subcommand: Option<DescribeSubCommands>
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
