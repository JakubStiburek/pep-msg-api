use crate::components::users::prelude::UserId;

pub struct UserNotFoundError {
    pub id: UserId,
    pub message: String,
}

impl UserNotFoundError {
    pub fn new(id: UserId) -> Self {
        Self {
            id: id.clone(),
            message: format!("User with id: \"{}\" not found", id.value),
        }
    }
}
