use crate::application::manager::output_manager::OutputManager;

pub struct OutputManagerImpl {}

impl OutputManagerImpl {
    pub fn new() -> OutputManagerImpl {
        OutputManagerImpl {}
    }
}

impl OutputManager for OutputManagerImpl {
    fn output(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, error: &str) {
        eprintln!("{}", error);
    }
}
