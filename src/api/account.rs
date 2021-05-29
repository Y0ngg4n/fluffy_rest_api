use actix_web::{get, post, Responder, HttpRequest, web, HttpResponse};
use crate::db::models::user::{InputUser, NewUser, LoginUser, ReadUser};
use crate::db::account;
use scylla::{Session, IntoTypedRows};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use chrono::{DateTime, Duration, DurationRound};
use chrono::prelude::*;
use uuid::Uuid;
use crate::utils::jwt::create_jwt;
use crate::middlewares::auth::AuthorizationService;
use actix_web::dev::ResponseBody;
use serde::{Deserialize, Serialize};
use actix_web::error::{ResponseError, ErrorUnauthorized};
use actix_web::client::HttpError;
use actix_web::{http::StatusCode};
use futures::future::{err, ok, Ready};
use scylla::macros::FromRow;
use scylla::cql_to_rust::FromRowError;

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    auth_token: String,
    name: String,
}


#[post("/login")]
pub async fn login(user: web::Json<LoginUser>, session: web::Data<Arc<Session>>) -> impl Responder {
    if let Some(rows) = account::get_user_by_email(&session, user.email.clone()).await {
        if rows.is_empty() {
            HttpResponse::Conflict().body("User does not exist")
        } else {
            let row = rows.into_typed::<ReadUser>().next();
            let read_row = row.unwrap().unwrap();
            let hashed_password = read_row.password;
            println!("{}", &hashed_password);
            // Argon2 with default params (Argon2id v19)
            let argon2 = Argon2::default();
            // Verify password against PHC string
            let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
            if argon2.verify_password(user.password.as_ref(),
                                      &parsed_hash).is_ok() {
                let token = create_jwt(read_row.id.to_string());
                HttpResponse::Ok().json(LoginResponse {
                    auth_token: token.to_string(),
                    name: read_row.name,
                })
            } else { HttpResponse::BadRequest().body("Wrong Password") }
        }
    } else {
        HttpResponse::Conflict().body("User does not exist")
    }
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    auth_token: String,
}

#[post("/register")]
pub async fn register(user: web::Json<InputUser>, session: web::Data<Arc<Session>>) -> impl Responder {
    let rows = account::get_user_by_email(&session, user.email.clone()).await;
    if !rows.unwrap().is_empty() {
        HttpResponse::Conflict().body("User does allready exist")
    } else {
        let salt = SaltString::generate(&mut OsRng);
        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();
        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password_simple(user.password.as_ref(),
                                                        salt.as_ref()).unwrap().to_string();
        // Verify password against PHC string
        let parsed_hash = PasswordHash::new(&password_hash).unwrap();
        assert!(argon2.verify_password(user.password.as_ref(), &parsed_hash).is_ok());
        let uid: Uuid = Uuid::new_v4();
        let new_user = NewUser {
            uuid: uid,
            name: user.name.clone(),
            password: password_hash,
            email: user.email.clone(),
            created: Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
        };
        let token = create_jwt(uid.to_string());
        account::add_user(&session, new_user).await.expect("Cant add User");
        HttpResponse::Ok().json(RegisterResponse {
            auth_token: token.to_string()
        })
    }
}

#[post("/account/delete")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}

#[post("/protectedRoute")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().json({ "test" })
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(protected);
}