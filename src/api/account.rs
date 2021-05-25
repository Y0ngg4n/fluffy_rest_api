use actix_web::{get, post, Responder, HttpRequest, web};
use crate::db::models::user::{InputUser, NewUser};
use crate::db::account;
use scylla::Session;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use rand_core::OsRng;
use std::sync::Arc;

#[get("/account/users")]
pub async fn get_users() -> impl Responder {
    format!("hello from get users")
}

#[get("/account/userbyid")]
pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

#[post("/account/add")]
pub async fn add_user(user: web::Json<InputUser>, session: web::Data<Arc<Session>>) -> impl Responder {

    // account::add_user(session,NewUser{
    //     name: user.name,
    //     email: user.email,
    //     password: String::from(""),
    //     created: String::from(""),
    // })
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password_simple(user.password.as_ref(),
                                                    salt.as_ref()).unwrap().to_string();
    // Verify password against PHC string
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    assert!(argon2.verify_password(user.password.as_ref(), &parsed_hash).is_ok());
    format!("hello from add user")
}

#[post("/account/delete")]
pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}