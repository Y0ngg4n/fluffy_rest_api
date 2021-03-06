use uuid::Uuid;
use scylla::frame::value::Timestamp;
use serde::{Deserialize, Serialize};
use chrono::Duration;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Serialize, Deserialize)]
pub struct InputGetDirectory{
    pub parent: String,
}

#[derive(FromRow)]
pub struct ReadGetDirectory{
    pub id:  Uuid,
    pub created: Duration,
    pub filename: String,
    pub owner: Uuid,
    pub parent: Uuid,
}

#[derive(Clone, Copy)]
pub struct NewGetDirectory{
    pub owner: Uuid,
    pub parent: Uuid,
}

#[derive(Clone, Copy)]
pub struct NewGetDirectoryAll{
    pub owner: Uuid,
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
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputMoveDirectory{
    pub id: Uuid,
    pub parent: String,
}

pub struct NewRenameDirectory{
    pub id: Uuid,
    pub filename: String,
}

pub struct NewMoveDirectory{
    pub id: Uuid,
    pub parent: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputDeleteDirectory{
    pub id: Uuid,
}

pub struct NewDeleteDirectory{
    pub id: Uuid,
}


#[derive(Serialize, Deserialize)]
pub struct InputGetWhiteboard{
    pub directory: String,
}

#[derive(Clone, Copy)]
pub struct NewGetWhiteboard{
    pub owner: Uuid,
    pub directory: Uuid,
}

#[derive(FromRow)]
pub struct ReadGetWhiteboard{
    pub id:  Uuid,
    pub created: Duration,
    pub directory: Uuid,
    pub edit_id: Uuid,
    pub name: String,
    pub owner: Uuid,
    pub password: String,
    pub view_id: Uuid,
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
    pub created: Timestamp,
    pub view_id: Uuid,
    pub edit_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputRenameWhiteboard{
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct InputMoveWhiteboard{
    pub id: Uuid,
    pub directory: String,
}

pub struct NewRenameWhiteboard{
    pub id: Uuid,
    pub name: String,
}

pub struct NewMoveWhiteboard{
    pub id: Uuid,
    pub directory: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputDeleteWhiteboard{
    pub id: Uuid,
}

pub struct NewDeleteWhiteboard{
    pub id: Uuid,
}