use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use crate::db::models::user::NewUser;
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::frame::response::result::Row;

pub async fn add_user(session_arc: &Arc<Session>, user: NewUser) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.account (id, name, email, password, created) \
            VALUES (?, ?, ?, ?, ?);",
            (user.id, user.name, user.email, user.password, user.created),
        )
        .await?;
    Ok(())
}

pub async fn get_user_by_email(session_arc: &Arc<Session>, email: String) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.account WHERE email=?;",
            (email, ),
        )
        .await.ok()?.rows
}

pub async fn get_user_by_id(session_arc: &Arc<Session>, id: Uuid) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.account WHERE id=?;",
            (id, ),
        )
        .await.ok()?.rows
}

pub async fn delete_user_by_id(session_arc: &Arc<Session>, id: Uuid) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.account WHERE id=?;",
            (id, ),
        )
        .await?;
    Ok(())
}

pub async fn update_username_by_id(session_arc: &Arc<Session>, name: String,
                                   id: Uuid, email: String) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.account SET name=? WHERE id=? AND email=?;",
            (name, id, email),
        )
        .await?;
    Ok(())
}
