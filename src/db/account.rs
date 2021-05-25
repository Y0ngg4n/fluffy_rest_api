use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use crate::db::models::user::NewUser;
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::frame::response::result::Row;

pub async fn add_user(session_arc: &Arc<Session>, user: NewUser) -> Result<(), Box<dyn Error>>{
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.account (id, name, email, password, created) \
            VALUES (?, ?, ?, ?, ?);",
            (user.uuid, user.name, user.email, user.password, user.created),
        )
        .await?;
    Ok(())
}

pub async fn get_user_by_email(session_arc: &Arc<Session>, email: String) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.account WHERE email=?;",
            (email,),
        )
        .await.ok()?.rows
}
