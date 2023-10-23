use clap::Args;

#[derive(Args)]
#[derive(Debug)]
#[command(about = "Create a license file")]
pub struct LicenseSubCommand {

}

impl LicenseSubCommand {
    pub fn create_license(&self) {
        println!("changelog called");
    }
}
