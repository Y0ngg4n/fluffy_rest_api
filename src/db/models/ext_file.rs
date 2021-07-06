use uuid::Uuid;
use scylla::frame::value::Timestamp;
use serde::{Deserialize, Serialize};
use chrono::Duration;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Serialize, Deserialize)]
pub struct InputGetExtWhiteboard{
    pub directory: String,
}

#[derive(Clone, Copy)]
pub struct NewGetOtherWhiteboard{

    pub id: Uuid,
}

#[derive(Clone, Copy)]
pub struct NewGetExtWhiteboard{
    pub account: Uuid,
    pub directory: Uuid,
}

#[derive(FromRow)]
pub struct ReadGetExtWhiteboard{
    pub id:  Uuid,
    pub account: Uuid,
    pub directory: Uuid,
    pub edit: bool,
    pub name: String,
    pub original: Uuid,
    pub permission_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputCreateExtWhiteboard{
    pub directory: String,
    pub id: Uuid,
    pub permission_id: Uuid,
}

pub struct NewCreateExtWhiteboard{
    pub id: Uuid,
    pub account: Uuid,
    pub directory: Uuid,
    pub name: String,
    pub edit: bool,
    pub original: Uuid,
    pub permission_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputDeleteExtWhiteboard{
    pub id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct InputMoveExtWhiteboard{
    pub id: Uuid,
    pub directory: String,
}

pub struct NewMoveExtWhiteboard{
    pub id: Uuid,
    pub directory: Uuid,
}

pub struct NewDeleteExtWhiteboard{
    pub id: Uuid,
}