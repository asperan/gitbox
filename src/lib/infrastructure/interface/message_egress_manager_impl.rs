use std::io::stdout;
use std::io::Write;

use crate::application::manager::message_egress_manager::MessageEgressManager;

pub struct MessageEgressManagerImpl {}

impl MessageEgressManagerImpl {
    pub fn new() -> MessageEgressManagerImpl {
        MessageEgressManagerImpl {}
    }
}

impl MessageEgressManager for MessageEgressManagerImpl {
    fn output(&self, message: &str) {
        if let Err(e) = writeln!(stdout(), "{}", message) {
            match e.kind() {
                std::io::ErrorKind::BrokenPipe => {}
                _ => self.error(&e.to_string()),
            }
        }
    }

    fn error(&self, error: &str) {
        eprintln!("{}", error);
    }
}
