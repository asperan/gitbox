pub trait UseCase<T, E> {
    fn execute(&self) -> Result<T, E>;
}
