use clap::Args;

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Generate a changelog")]
pub struct ChangelogSubCommand {

}

impl ChangelogSubCommand {
    pub fn changelog(&self) {
        println!("changelog called");
    }
}
