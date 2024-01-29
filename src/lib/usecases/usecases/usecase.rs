pub trait UseCase<T> {
    fn execute(&self) -> T;
}
