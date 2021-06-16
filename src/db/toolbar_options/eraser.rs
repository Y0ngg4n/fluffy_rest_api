use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::{NewUpdateHighlighter, NewUpdateEraser};

pub async fn get_eraser(session_arc: &Arc<Session>, uuid: Uuid) -> Option<Vec<Row>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.toolbar_options_eraser WHERE owner=?;",
            (uuid, ),
        )
        .await.ok()?.rows
}

pub async fn update_eraser(session_arc: &Arc<Session>, eraser: NewUpdateEraser) -> Result<(), Box<dyn Error>>  {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.toolbar_options_eraser (owner, stroke_width) VALUES (?, ?);",
            (eraser.owner, eraser.stroke_width),
        ).await?;
    Ok(())
}