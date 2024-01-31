pub struct InitOptions {
    empty: bool,
}

impl InitOptions {
    pub fn new(empty: bool) -> InitOptions {
        InitOptions { empty }
    }

    pub fn empty(&self) -> bool {
        self.empty
    }
}
