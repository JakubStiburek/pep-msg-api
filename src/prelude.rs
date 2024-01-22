pub use std::str::FromStr;

pub use actix_web::{App, get, HttpResponse, HttpServer, post, put, Responder, web};
pub use mongodb::{bson::doc, bson::oid::ObjectId, Client, Collection, error::ErrorKind::Command, error::ErrorKind::Write, IndexModel, options::IndexOptions};
pub use serde::{Deserialize, Serialize};

pub const DB_NAME: &str = "pep-msg";

