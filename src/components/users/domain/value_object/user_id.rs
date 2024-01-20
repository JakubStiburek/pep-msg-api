use crate::prelude::ObjectId;

#[derive(Clone, Debug)]
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
