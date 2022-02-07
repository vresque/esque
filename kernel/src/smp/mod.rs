pub struct Thread {
    func: fn()
}

impl Thread {
    pub fn new(func: fn()) -> Self {
        Self { func }
    }
    pub fn launch() {}
}