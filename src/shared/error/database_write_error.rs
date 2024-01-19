#[derive(Debug)]
pub struct DatabaseWriteError {
    pub message: String,
}

impl DatabaseWriteError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            message: message.to_string()
        }
    }
}
