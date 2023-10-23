use clap::Args;

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Print a completion script")]
pub struct CompleteSubCommand {

}

impl CompleteSubCommand {
    pub fn print_completion_script(&self) {
        println!("changelog called");
    }
}
