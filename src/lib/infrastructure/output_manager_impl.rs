use crate::application::manager::message_egress_manager::MessageEgressManager;

pub struct MessageEgressManagerImpl {}

impl MessageEgressManagerImpl {
    pub fn new() -> MessageEgressManagerImpl {
        MessageEgressManagerImpl {}
    }
}

impl MessageEgressManager for MessageEgressManagerImpl {
    fn output(&self, message: &str) {
        println!("{}", message);
    }

    fn error(&self, error: &str) {
        eprintln!("{}", error);
    }
}
