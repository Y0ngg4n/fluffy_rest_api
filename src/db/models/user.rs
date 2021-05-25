use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser{
    pub name: String,
    pub email: String,
    pub password: String,
}