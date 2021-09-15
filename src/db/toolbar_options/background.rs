use std::sync::Arc;
use scylla::Session;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateBackground};

pub async fn get_background(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_background WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_background(session_arc: &Arc<Session>, background: NewUpdateBackground) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_background (owner, color_presets, stroke_width, selected_background) VALUES (?, ?, ?, ?);",
            (background.owner, background.color_presets, background.stroke_width, background.selected_background),
        ).await?;

    Ok(())
}