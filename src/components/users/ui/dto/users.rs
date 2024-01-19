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
    pub fn from_entity(entity: User) -> Self {
        Self {
            id: entity.id.value,
            username: entity.username.value,
        }
    }
}

// #[derive(Serialize)]
// pub struct UserIdDto {
//     pub value: String,
// }

// impl UserIdDto {
//     pub fn from_entity(entity: UserId) -> Self {
//         Self {
//             value: entity.value
//         }
//     }
// }
