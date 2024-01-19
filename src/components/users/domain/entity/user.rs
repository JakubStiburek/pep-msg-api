use crate::components::users::prelude::*;

pub struct User {
    pub id: UserId,
    pub username: Username,
}

impl User {
    pub fn new(id: UserId, username: Username) -> Self {
        Self {
            id,
            username,
        }
    }
}

pub trait UserOperations {
    async fn get_all(&self) -> Result<Vec<User>, InfrastructureError>;
    async fn create(&self, username: Username) -> Result<User, InfrastructureError>;
}
