use std::sync::Arc;
use scylla::Session;
use uuid::Uuid;
use crate::api::websocket::json_messages::{TextItemAdd, TextItemUpdate, TextItemDelete};

pub async fn text_item_add(session: Arc<Session>, text_item: TextItemAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_textitem (id, whiteboard, stroke_width, max_width, max_height, color, content_text, offset_dx, offset_dy, rotation) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            (text_item.uuid, whiteboard, text_item.stroke_width, text_item.max_width, text_item.max_height, text_item.color, text_item.content_text, text_item.offset_dx, text_item.offset_dy, text_item.rotation),
        ).await.expect("Could not insert textitem");
}

pub async fn text_item_update(session: Arc<Session>, text_item: TextItemUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_textitem SET stroke_width=?, max_width=?, max_height=?, color=?, content_text=?, offset_dx=?, offset_dy=?, rotation=? WHERE id=?",
            (text_item.stroke_width, text_item.max_width, text_item.max_height, text_item.color, text_item.content_text, text_item.offset_dx, text_item.offset_dy, text_item.rotation, text_item.uuid),
        ).await.expect("Could not update textitem");
}

pub async fn text_item_delete(session: Arc<Session>, scribble: TextItemDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_textitem WHERE id=?",
            (scribble.uuid, ),
        ).await.expect("Could not delete scribble");
}




