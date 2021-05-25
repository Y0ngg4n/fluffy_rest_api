use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use crate::db::models::user::NewUser;

pub async fn add_user(session: Session, user: NewUser) -> Result<(), Box<dyn Error>>{
    session
        .query(
            "INSERT INTO fluffy_board.account (name, email, password, created) \
            VALUES (?, ?, ?, ?)
            )",
            &(user.name, user.email, user.password, user.created),
        )
        .await?;
    Ok(())
}
