use futures_util::TryStreamExt;
use mongodb::error::Error;

use crate::components::users::prelude::*;
use crate::prelude::*;
use crate::shared::prelude::*;

pub const COLL_NAME: &str = "users";

pub struct UserRepository<'a> {
    pub client: &'a web::Data<Client>,
    collection: Collection<UserModel>,
}

impl<'a> UserRepository<'a> {
    pub fn new(client: &'a web::Data<Client>) -> Self {
        Self {
            client,
            collection: client.database(DB_NAME).collection(COLL_NAME),
        }
    }
}

impl<'a> UserOperations for UserRepository<'a> {
    async fn get_all(&self) -> Result<Vec<User>, InfrastructureError> {
        let result = self.collection.find(None, None).await;
        match result {
            Ok(cursor) => {
                let documents: Result<Vec<UserModel>, Error> = cursor.try_collect().await;
                match documents {
                    Ok(documents) => Ok(documents.into_iter().map(|doc| User::new(UserId::new(doc._id), Username::new(doc.username))).collect()),
                    Err(_) => Err(InfrastructureError::Unhandled(GenericError::new("Unhandled mongo error.")))
                }
            }
            Err(_) => Err(InfrastructureError::Unhandled(GenericError::new("Unhandled mongo error.")))
        }
    }
    async fn create(&self, username: Username) -> Result<User, InfrastructureError> {
        let result = self.collection.insert_one(UserModel { _id: ObjectId::new(), username: username.value.clone() }, None).await;
        match result {
            Ok(res) => {
                match res.inserted_id.as_object_id() {
                    Some(object_id) => Ok(User::new(UserId::new(object_id.clone()), Username::new(username.value))),
                    None => Err(InfrastructureError::Unhandled(GenericError::new("Inserted id is not an ObjectId".to_string()))),
                }
            }
            Err(err) => {
                match *err.kind {
                    Write(_) => Err(InfrastructureError::Write(DatabaseWriteError::new(err.to_string()))),
                    _ => Err(InfrastructureError::Unhandled(GenericError::new("Unhandled mongo error.")))
                }
            }
        }
    }
}
