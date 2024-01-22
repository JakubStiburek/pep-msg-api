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
    async fn get_all(&self) -> Result<Vec<User>, DatabaseError>;
    async fn get_by_id(&self, id: UserId) -> Result<User, DatabaseError>;
    async fn create(&self, username: Username) -> Result<User, DatabaseError>;
    async fn update(&self, id: UserId, username: Username) -> Result<User, DatabaseError>;
}
