use crate::prelude::ObjectId;

pub struct UserId {
    pub value: ObjectId,
}

impl UserId {
    pub fn new(value: ObjectId) -> Self {
        Self {
            value
        }
    }
}
