use std::sync::Arc;
use scylla::Session;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateFigure};

pub async fn get_figure(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_figure WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_figure(session_arc: &Arc<Session>, figure: NewUpdateFigure) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_figure (owner, color_presets, stroke_width, selected_color, selected_figure, selected_fill) VALUES (?, ?, ?, ?, ?, ?);",
            (figure.owner, figure.color_presets, figure.stroke_width, figure.selected_color, figure.selected_figure, figure.selected_fill),
        ).await?;
    Ok(())
}