pub trait MessageEgressManager {
    fn output(&self, message: &str);

    fn error(&self, error: &str);
}
