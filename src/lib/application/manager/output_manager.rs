pub trait OutputManager {
    fn output(&self, message: &str);

    fn error(&self, error: &str);
}
