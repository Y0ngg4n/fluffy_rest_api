use std::sync::Arc;
use scylla::Session;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateTextItem};

pub async fn get_text_item(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_text_item WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_text_item(session_arc: &Arc<Session>, text_item: NewUpdateTextItem) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_text_item (owner, color_presets, stroke_width, selected_color) VALUES (?, ?, ?, ?, ?);",
            (text_item.owner, text_item.color_presets, text_item.stroke_width, text_item.selected_color),
        ).await?;
    Ok(())
}