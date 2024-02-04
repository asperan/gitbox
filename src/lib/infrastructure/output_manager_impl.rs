use crate::application::manager::output_manager::MessageEgressManager;

pub struct OutputManagerImpl {}

impl OutputManagerImpl {
    pub fn new() -> OutputManagerImpl {
        OutputManagerImpl {}
    }
}

impl MessageEgressManager for OutputManagerImpl {
    fn output(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, error: &str) {
        eprintln!("{}", error);
    }
}
