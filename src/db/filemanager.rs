use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::frame::response::result::Row;
use crate::db::models::file::{NewCreateDirectory, InputRenameDirectory, NewRenameDirectory, InputDeleteDirectory, NewCreateWhiteboard, NewRenameWhiteboard, InputDeleteWhiteboard, NewGetDirectory, NewGetWhiteboard, NewDeleteDirectory, ReadGetWhiteboard, NewDeleteWhiteboard};

pub async fn get_directory(session_arc: &Arc<Session>, directory: NewGetDirectory) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.wb_directory WHERE owner=? AND parent=?;",
            (directory.owner, directory.parent),
        )
        .await.ok()?.rows
}

pub async fn create_directory(session_arc: &Arc<Session>, directory: NewCreateDirectory) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.wb_directory (id, owner, parent, filename, created) \
            VALUES (?, ?, ?, ?, ?);",
            (directory.id, directory.owner, directory.parent, directory.filename, directory.created),
        )
        .await?;
    Ok(())
}

pub async fn rename_directory(session_arc: &Arc<Session>, directory: NewRenameDirectory) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.wb_directory SET filename=? WHERE id=? AND owner=? AND parent=?;",
            (directory.filename, directory.id, directory.owner, directory.parent),
        )
        .await?;
    Ok(())
}

pub async fn delete_directory(session_arc: &Arc<Session>, directory: NewDeleteDirectory) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.wb_directory WHERE id=?;",
            (directory.id, ),
        )
        .await?;
    Ok(())
}

pub async fn get_whiteboard(session_arc: &Arc<Session>, whiteboard: NewGetWhiteboard) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.whiteboard WHERE owner=? AND directory=?;",
            (whiteboard.owner, whiteboard.directory),
        )
        .await.ok()?.rows
}

pub async fn create_whiteboard(session_arc: &Arc<Session>, whiteboard: NewCreateWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.whiteboard (id, owner, directory, name, password, created,\
            edit_id, view_id, data) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);",
            (whiteboard.id, whiteboard.owner, whiteboard.directory, whiteboard.name,
             whiteboard.password, whiteboard.created, whiteboard.edit_id, whiteboard.view_id, whiteboard.data),
        )
        .await?;
    Ok(())
}

pub async fn rename_whiteboard(session_arc: &Arc<Session>, whiteboard: NewRenameWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.whiteboard SET name=? WHERE id=? AND owner=? AND directory=?;",
            (whiteboard.name, whiteboard.id, whiteboard.owner, whiteboard.directory),
        )
        .await?;
    Ok(())
}

pub async fn delete_whiteboard(session_arc: &Arc<Session>, whiteboard: NewDeleteWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.whiteboard WHERE id=?;",
            (whiteboard.id, ),
        )
        .await?;
    Ok(())
}