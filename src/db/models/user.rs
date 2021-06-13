use serde::{Deserialize, Serialize};
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use chrono::Duration;

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    pub password: String,
    pub email: String,
}

// #[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created: Timestamp,
}


#[derive(FromRow)]
pub struct ReadUser {
    pub id: Uuid,
    pub created: Duration,
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputUser{
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNameInput{
    pub name: String,
    pub email: String,
}