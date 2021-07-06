use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post, get, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::auth::AuthorizationService;
use crate::db::models::file::InputGetDirectory;
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::ext_file::{InputCreateExtWhiteboard, NewCreateExtWhiteboard, ReadGetExtWhiteboard, NewGetExtWhiteboard, NewGetOtherWhiteboard, NewDeleteExtWhiteboard, NewMoveExtWhiteboard};
use chrono::format::Numeric::Timestamp;
use chrono::Duration;
use crate::api::filemanager::{parse_dir_uuid, parse_own_uuid};
use std::error::Error;
use crate::db::filemanager::get_directory;
use scylla::frame::response::result::Row;


pub async fn get_other_whiteboard(session_arc: &Arc<Session>, whiteboard: NewGetOtherWhiteboard) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.whiteboard WHERE id=?;",
            (whiteboard.id,),
        )
        .await.ok()?.rows
}

pub async fn get_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewGetExtWhiteboard) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.ext_whiteboard WHERE account=? AND directory=? ALLOW FILTERING;",
            (whiteboard.account, whiteboard.directory),
        )
        .await.ok()?.rows
}

pub async fn create_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewCreateExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);

        session
            .query(
                "INSERT INTO fluffy_board.ext_whiteboard (id, account, directory, name, edit, original, permission_id) \
            VALUES (?, ?, ?, ?, ?, ?, ?);",
                (whiteboard.id, whiteboard.account, whiteboard.directory, whiteboard.name,
                 whiteboard.edit, whiteboard.original, whiteboard.permission_id)
            ).await?;
    Ok(())
}

pub async fn move_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewMoveExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.ext_whiteboard SET directory=? WHERE id=?;",
            (whiteboard.directory, whiteboard.id),
        )
        .await?;
    Ok(())
}

pub async fn delete_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewDeleteExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.ext_whiteboard WHERE id=?;",
            (whiteboard.id, ),
        )
        .await?;
    Ok(())
}