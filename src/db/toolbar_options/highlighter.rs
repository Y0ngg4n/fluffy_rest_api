use std::sync::Arc;
use scylla::Session;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateHighlighter};

pub async fn get_highlighter(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_highlighter WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_highlighter(session_arc: &Arc<Session>, highlighter: NewUpdateHighlighter) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_highlighter (owner, color_presets, stroke_width, selected_color) VALUES (?, ?, ?, ?);",
            (highlighter.owner, highlighter.color_presets, highlighter.stroke_width, highlighter.selected_color),
        ).await?;
    Ok(())
}