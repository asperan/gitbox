use clap::{Args, Command};
use clap_complete::Shell;

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Print a completion script", after_help = "This subcommand prints to stdout. The output can be redirected to a file and the file can be sourced during shell initialization to always provide completion.")]
pub struct CompleteSubCommand {
    #[arg(short, long, help = "Set the shell for which print the completion script", value_parser = clap::builder::EnumValueParser::<Shell>::new(), default_value = "bash" )]
    shell: Shell,
}

impl CompleteSubCommand {
    pub fn print_completion_script(&self, entrypoint: &mut Command) {
        clap_complete::generate(self.shell, entrypoint, entrypoint.get_name().to_string(), &mut std::io::stdout());
    }
}
