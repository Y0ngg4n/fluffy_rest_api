use std::sync::Arc;
use scylla::Session;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::NewUpdatePencil;

pub async fn get_pencil(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_pencil WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_pencil(session_arc: &Arc<Session>, pencil: NewUpdatePencil) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_pencil (owner, color_presets, stroke_width, selected_color) VALUES (?, ?, ?, ?);",
            (pencil.owner, pencil.color_presets, pencil.stroke_width, pencil.selected_color),
        ).await?;
    Ok(())
}