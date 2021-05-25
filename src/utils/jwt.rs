use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;
use std::{env, io};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AccountToken {
    sub: String,
    exp: usize,
}

pub fn create_jwt(uuid: String) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp();

    let account_token = AccountToken {
        sub: uuid,
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &account_token, &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_ref())).expect("Could not create JWT")
}