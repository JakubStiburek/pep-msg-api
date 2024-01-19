pub struct Username {
    pub value: String,
}

impl Username {
    pub fn new(value: impl ToString) -> Self {
        Self {
            value: value.to_string()
        }
    }
}
