use uuid::Uuid;
use scylla::frame::value::Timestamp;
use serde::{Deserialize, Serialize};
use chrono::Duration;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Serialize, Deserialize)]
pub struct InputGetDirectory{
    pub parent: String,
}

#[derive(FromRow)]
pub struct ReadGetDirectory{
    pub id:  Uuid,
    pub owner: Uuid,
    pub parent: Uuid,
    pub created: Duration,
    pub filename: String,
}

pub struct NewGetDirectory{
    pub owner: Uuid,
    pub parent: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputCreateDirectory{
    pub parent: String,
    pub filename: String,
}

pub struct NewCreateDirectory{
    pub id: Uuid,
    pub owner: Uuid,
    pub parent: Uuid,
    pub filename: String,
    pub created: Timestamp
}

#[derive(Serialize, Deserialize)]
pub struct InputRenameDirectory{
    pub id: Uuid,
    pub parent: Uuid,
    pub filename: String,
}

pub struct NewRenameDirectory{
    pub id: Uuid,
    pub owner: Uuid,
    pub parent: Uuid,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputDeleteDirectory{
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputGetWhiteboard{
    pub directory: String,
}

pub struct NewGetWhiteboard{
    pub owner: Uuid,
    pub directory: Uuid,
}

#[derive(FromRow)]
pub struct ReadGetWhiteboard{
    pub id:  Uuid,
    pub owner: Uuid,
    pub directory: Uuid,
    pub created: Duration,
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputCreateWhiteboard{
    pub name: String,
    pub directory: String,
    pub password: String,
}

pub struct NewCreateWhiteboard{
    pub id: Uuid,
    pub owner: Uuid,
    pub name: String,
    pub directory: Uuid,
    pub password: String,
    pub created: Timestamp
}

#[derive(Serialize, Deserialize)]
pub struct InputRenameWhiteboard{
    pub id: Uuid,
    pub directory: Uuid,
    pub name: String,
}

pub struct NewRenameWhiteboard{
    pub id: Uuid,
    pub owner: Uuid,
    pub directory: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputDeleteWhiteboard{
    pub id: Uuid,
}