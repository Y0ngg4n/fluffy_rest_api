use std::sync::Arc;
use scylla::Session;
use uuid::Uuid;
use crate::api::websocket::json_messages::{BookmarkAdd, BookmarkUpdate, BookmarkDelete};

pub async fn bookmark_add(session: Arc<Session>, bookmark: BookmarkAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_bookmark (id, whiteboard, name, offset_dx, offset_dy, scale) VALUES (?, ?, ?, ?, ?, ?);",
            (bookmark.uuid, whiteboard, bookmark.name, bookmark.offset_dx, bookmark.offset_dy, bookmark.scale)
        ).await.expect("Could not insert bookmark");
}

pub async fn bookmark_update(session: Arc<Session>, bookmark: BookmarkUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_bookmark SET name=?, offset_dx=?, offset_dy=?, scale=? WHERE id=?",
            (bookmark.name, bookmark.offset_dx, bookmark.offset_dy, bookmark.scale, bookmark.uuid,)
        ).await.expect("Could not update bookmark");
}

pub async fn bookmark_delete(session: Arc<Session>, bookmark: BookmarkDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_bookmark WHERE id=?",
            (bookmark.uuid,)
        ).await.expect("Could not delete bookmark");
}
