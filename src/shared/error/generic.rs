#[derive(Debug)]
pub struct GenericError {
    pub message: String,
}

impl GenericError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string()
        }
    }
}
