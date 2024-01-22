use crate::components::users::prelude::*;
use crate::shared::prelude::*;

pub enum DatabaseError {
    Unhandled(GenericError),
    Write(DatabaseWriteError),
    Update(GenericError),
    UserNotFound(UserNotFoundError),
}
