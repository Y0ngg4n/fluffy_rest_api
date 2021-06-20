use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use crate::utils::jwt;
use std::env;
use uuid::Uuid;

pub struct AuthResult{
    pub authenticated: bool,
    pub uuid: Uuid,
}

pub(crate) fn check_auth(token: &str) -> AuthResult {
    let _var = env::var("JWT_AUTH_SECRET").unwrap();
    let key = _var.as_bytes();
    match decode::<jwt::AccountToken>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(_token) => AuthResult{
            authenticated: true,
            uuid: Uuid::parse_str(_token.claims.sub.as_str()).unwrap()
        },
        Err(_e) => AuthResult{
            authenticated: false,
            uuid: Uuid::nil()
        },
    }
}
