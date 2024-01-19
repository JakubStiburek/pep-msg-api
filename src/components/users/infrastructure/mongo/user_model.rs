use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserModel {
    pub _id: ObjectId,
    pub username: String,
}
