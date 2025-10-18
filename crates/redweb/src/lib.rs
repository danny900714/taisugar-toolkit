pub struct RedWeb {
    origin: String,
}

impl RedWeb {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
