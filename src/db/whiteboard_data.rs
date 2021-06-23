use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::NewUpdatePencil;
use crate::db::models::whiteboard::{InputGetWhiteboardScribble, InputGetWhiteboardUpload};

pub async fn get_whiteboard_by_id(session_arc: &Arc<Session>, whiteboard: Uuid)-> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.whiteboard WHERE id=?",
            (whiteboard,)
        ).await.ok()?.rows
}

pub async fn get_whiteboard_scribbles(session_arc: &Arc<Session>, scribble: InputGetWhiteboardScribble)-> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.wb_scribble WHERE whiteboard=?",
            (scribble.whiteboard,)
        ).await.ok()?.rows
}

pub async fn get_whiteboard_upload(session_arc: &Arc<Session>, scribble: InputGetWhiteboardUpload)-> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.wb_upload WHERE whiteboard=?",
            (scribble.whiteboard,)
        ).await.ok()?.rows
}