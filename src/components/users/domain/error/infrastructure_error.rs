use crate::shared::prelude::*;

pub enum InfrastructureError {
    Unhandled(GenericError),
    Write(DatabaseWriteError),
}
