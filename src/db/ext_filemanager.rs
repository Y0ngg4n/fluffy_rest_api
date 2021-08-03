use std::sync::Arc;
use scylla::{Session};
use crate::db::models::ext_file::{NewCreateExtWhiteboard, NewGetExtWhiteboard, NewGetOtherWhiteboard, NewDeleteExtWhiteboard, NewMoveExtWhiteboard};
use std::error::Error;
use scylla::frame::response::result::Row;


pub async fn get_other_whiteboard(session_arc: &Arc<Session>, whiteboard: NewGetOtherWhiteboard) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.whiteboard WHERE id=?;",
            (whiteboard.id,),
        )
        .await.ok()?.rows
}

pub async fn get_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewGetExtWhiteboard) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.ext_whiteboard WHERE account=? AND directory=? ALLOW FILTERING;",
            (whiteboard.account, whiteboard.directory),
        )
        .await.ok()?.rows
}

pub async fn create_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewCreateExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);

        session
            .query(
                "INSERT INTO fluffy_board.ext_whiteboard (id, account, directory, name, edit, original, permission_id) \
            VALUES (?, ?, ?, ?, ?, ?, ?);",
                (whiteboard.id, whiteboard.account, whiteboard.directory, whiteboard.name,
                 whiteboard.edit, whiteboard.original, whiteboard.permission_id)
            ).await?;
    Ok(())
}

pub async fn move_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewMoveExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.ext_whiteboard SET directory=? WHERE id=?;",
            (whiteboard.directory, whiteboard.id),
        )
        .await?;
    Ok(())
}

pub async fn delete_ext_whiteboard(session_arc: &Arc<Session>, whiteboard: NewDeleteExtWhiteboard) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "DELETE FROM fluffy_board.ext_whiteboard WHERE id=?;",
            (whiteboard.id, ),
        )
        .await?;
    Ok(())
}