use mongodb::bson;

use crate::components::users::prelude::*;
use crate::prelude::*;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub username: String,
}

// #[derive(Deserialize)]
// pub struct UpdateUserDto {
//     pub username: String,
// }

#[derive(Serialize)]
pub struct UserDto {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
}

impl UserDto {
    pub fn from_domain(entity: User) -> Self {
        Self {
            id: entity.id.value,
            username: entity.username.value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserIdDto {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub value: ObjectId,
}

impl UserIdDto {
    // pub fn from_domain(entity: UserId) -> Self {
    //     Self {
    //         value: entity.value
    //     }
    // }
    pub fn from_string(id: String) -> Option<Self> {
        match ObjectId::from_str(id.as_str()) {
            Ok(id) => Some(Self { value: id }),
            Err(_) => None
        }
    }
    pub fn to_domain(&self) -> UserId {
        UserId::new(self.value)
    }
}
