use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::NewUpdatePencil;
use crate::api::websocket::json_messages::{ScribbleAdd, ScribbleUpdate, ScribbleDelete, TextItemAdd, TextItemUpdate, TextItemDelete};

pub async fn text_item_add(session: Arc<Session>, textItem: TextItemAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_textitem (id, whiteboard, stroke_width, max_width, max_height, color, content_text, offset_dx, offset_dy) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);",
            (textItem.uuid, whiteboard, textItem.stroke_width, textItem.max_width, textItem.max_height, textItem.color, textItem.content_text, textItem.offset_dx, textItem.offset_dy)
        ).await.expect("Could not insert textitem");
}

pub async fn text_item_update(session: Arc<Session>, textItem: TextItemUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_textitem SET stroke_width=?, max_width=?, max_height=?, color=?, content_text=?, offset_dx=?, offset_dy=? WHERE id=?",
            (textItem.stroke_width, textItem.max_width, textItem.max_height, textItem.color, textItem.content_text, textItem.offset_dx, textItem.offset_dy, textItem.uuid)
        ).await.expect("Could not update textitem");
}

pub async fn text_item_delete(session: Arc<Session>, scribble: TextItemDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_textitem WHERE id=?",
            (scribble.uuid,)
        ).await.expect("Could not delete scribble");
}

pub async fn text_item_delete_whiteboard(session_arc: &Arc<Session>, uuid: &Uuid) {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.wb_textitem WHERE whiteboard=?",
            (uuid,)
        ).await.expect("Could not delete textitem");
}




