use actix_web::{get, post, Responder, web, HttpResponse};
use crate::db::models::user::{InputUser, NewUser, LoginUser, ReadUser, UpdateNameInput};
use crate::db::account;
use scylla::{Session, IntoTypedRows};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use chrono::{Duration};
use chrono::prelude::*;
use uuid::Uuid;
use crate::utils::jwt::create_auth_jwt;
use crate::middlewares::auth::AuthorizationService;
use serde::{Deserialize, Serialize};
use crate::db::account::{delete_user_by_id, update_username_by_id};

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    id: String,
    auth_token: String,
    name: String,
    email: String,
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
            // Argon2 with default params (Argon2id v19)
            let argon2 = Argon2::default();
            // Verify password against PHC string
            let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
            if argon2.verify_password(user.password.as_ref(),
                                      &parsed_hash).is_ok() {
                let token = create_auth_jwt(read_row.id.to_string());
                HttpResponse::Ok().json(LoginResponse {
                    id: read_row.id.to_string(),
                    auth_token: token.to_string(),
                    name: read_row.name,
                    email: read_row.email,
                })
            } else { HttpResponse::BadRequest().body("Wrong Password") }
        }
    } else {
        HttpResponse::Conflict().body("User does not exist")
    }
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    id: String,
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
            id: uid,
            name: user.name.clone(),
            password: password_hash,
            email: user.email.clone(),
            created: Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
        };
        let token = create_auth_jwt(uid.to_string());
        account::add_user(&session, new_user).await.expect("Cant add User");
        HttpResponse::Ok().json(RegisterResponse {
            id: uid.to_string(),
            auth_token: token.to_string()
        })
    }
}

#[post("/delete")]
pub async fn delete(auth: AuthorizationService, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    delete_user_by_id(&session, uuid).await.expect("Could not update Account");
    HttpResponse::Ok().body("Account deleted")
}

#[post("/update/username")]
pub async fn update_username(auth: AuthorizationService, user: web::Json<UpdateNameInput>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_username_by_id(&session, user.name.clone(), uuid).await.expect("Could not update Account");
    HttpResponse::Ok().body("Username updated")
}

#[get("/check")]
async fn protected(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().body("Success")
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(delete);
    cfg.service(update_username);
    cfg.service(protected);
}