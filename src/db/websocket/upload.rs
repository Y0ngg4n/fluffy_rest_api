use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::NewUpdatePencil;
use crate::api::websocket::json_messages::{ScribbleAdd, ScribbleUpdate, ScribbleDelete, UploadAdd, UploadUpdate};

pub async fn upload_add(session: Arc<Session>, upload: UploadAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_upload (id, whiteboard, upload_type, offset_dx, offset_dy, image_data) VALUES (?, ?, ?, ?, ?, ?);",
            (upload.uuid, whiteboard, upload.upload_type, upload.offset_dx, upload.offset_dy, upload.image_data)
        ).await.expect("Could not insert scribble");
}

pub async fn upload_update(session: Arc<Session>, upload: UploadUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_upload SET offset_dx=?, offset_dy=? WHERE id=?",
            (upload.offset_dx, upload.offset_dy, upload.uuid)
        ).await.expect("Could not update upload");
}

pub async fn upload_delete(session: Arc<Session>, scribble: ScribbleDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_scribble WHERE id=?",
            (scribble.uuid,)
        ).await.expect("Could not delete scribble");
}

