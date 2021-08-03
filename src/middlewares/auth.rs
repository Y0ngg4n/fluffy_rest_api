use actix_web::error::ErrorUnauthorized;
use actix_web::{dev, Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, TokenData};
use std::env;
use crate::utils::jwt;
use crate::utils::jwt::AccountToken;

pub struct AuthorizationService {
    pub(crate) token: TokenData<AccountToken>
}

impl FromRequest for AuthorizationService {
    type Error = Error;
    type Future = Ready<Result<AuthorizationService, Error>>;
    type Config = ();

    fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let _auth = _req.headers().get("Authorization");
        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();
                let _var = env::var("JWT_AUTH_SECRET").unwrap();
                let key = _var.as_bytes();
                match decode::<jwt::AccountToken>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS512),
                ) {
                    Ok(_token) => ok(AuthorizationService{token: _token}),
                    Err(_e) => err(ErrorUnauthorized(_e.to_string())),
                }
            }
            None => err(ErrorUnauthorized("blocked!")),
        }
    }
}