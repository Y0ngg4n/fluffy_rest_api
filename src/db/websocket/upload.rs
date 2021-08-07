use std::sync::Arc;
use scylla::Session;
use uuid::Uuid;
use crate::api::websocket::json_messages::{UploadAdd, UploadUpdate, UploadDelete, UploadImageDataUpdate};

pub async fn upload_add(session: Arc<Session>, upload: UploadAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_upload (id, whiteboard, upload_type, offset_dx, offset_dy, rotation, scale, image_data) VALUES (?, ?, ?, ?, ?, ?, ?);",
            (upload.uuid, whiteboard, upload.upload_type, upload.offset_dx, upload.offset_dy, upload.rotation, upload.scale, upload.image_data)
        ).await.expect("Could not insert scribble");
}

pub async fn upload_update(session: Arc<Session>, upload: UploadUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_upload SET offset_dx=?, offset_dy=? rotation=? scale=? WHERE id=?",
            (upload.offset_dx, upload.offset_dy, upload.rotation, upload.scale, upload.uuid)
        ).await.expect("Could not update upload");
}

pub async fn upload_image_data_update(session: Arc<Session>, upload: UploadImageDataUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_upload SET image_data=? WHERE id=?",
            (upload.image_data, upload.uuid)
        ).await.expect("Could not update upload");
}

pub async fn upload_delete(session: Arc<Session>, upload: UploadDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_upload WHERE id=?",
            (upload.uuid,)
        ).await.expect("Could not delete upload");
}

