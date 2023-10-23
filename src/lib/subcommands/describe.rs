use clap::{Args, Subcommand};

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
            None => println!("Default action"),
        }
    }
}

#[derive(Subcommand, Clone)]
#[derive(Debug)]
enum DescribeSubCommands {
    #[command(about = "TODO")]
    Docker(DescribeDockerSubCommand),
}

#[derive(Args, Clone)]
#[derive(Debug)]
struct DescribeDockerSubCommand {

}

impl DescribeDockerSubCommand {
    pub fn describe_docker(&self) {
        println!("describe docker called");
    }
}
