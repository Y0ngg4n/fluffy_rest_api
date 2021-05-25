use serde::{Deserialize, Serialize};
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub password: String,
    pub email: String,
}

// #[derive(Serialize, Deserialize)]
#[derive(FromRow)]
pub struct NewUser {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created: Timestamp,
}

#[derive(Serialize, Deserialize)]
pub struct InputUser{
    pub name: String,
    pub email: String,
    pub password: String,
}