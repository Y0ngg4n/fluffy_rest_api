use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateStraightLine};

pub async fn get_straight_line(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_straight_line WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_straight_line(session_arc: &Arc<Session>, straight_line: NewUpdateStraightLine) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_straight_line (owner, color_presets, stroke_width, selected_color, selected_cap) VALUES (?, ?, ?, ?, ?);",
            (straight_line.owner, straight_line.color_presets, straight_line.stroke_width, straight_line.selected_color, straight_line.selected_cap),
        ).await?;
    Ok(())
}