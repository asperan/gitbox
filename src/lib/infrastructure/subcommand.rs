pub trait Subcommand {
    fn execute(&self) -> i32;
}
