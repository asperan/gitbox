use clap::Args;

#[derive(Args, Clone)]
#[derive(Debug)]
pub struct DescribeDockerSubCommand {

}

impl DescribeDockerSubCommand {
    pub fn describe_docker(&self) {
        println!("describe docker called");
    }
}
