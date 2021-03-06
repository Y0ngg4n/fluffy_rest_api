use std::sync::Arc;
use scylla::Session;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::whiteboard::{InputGetWhiteboardScribble, InputGetWhiteboardUpload, InputGetWhiteboardTextItem, InputGetWhiteboardBookmark};

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

pub async fn get_whiteboard_text_item(session_arc: &Arc<Session>, textitem: InputGetWhiteboardTextItem)-> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.wb_textitem WHERE whiteboard=?",
            (textitem.whiteboard,)
        ).await.ok()?.rows
}

pub async fn get_whiteboard_bookmark(session_arc: &Arc<Session>, bookmark: InputGetWhiteboardBookmark)-> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.wb_bookmark WHERE whiteboard=?",
            (bookmark.whiteboard,)
        ).await.ok()?.rows
}