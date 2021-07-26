use scylla::{IntoTypedRows, Session, SessionBuilder, QueryResult};
use std::error::Error;
use scylla::transport::errors::{NewSessionError, QueryError};
use crate::db::models::user::NewUser;
use std::sync::Arc;
use scylla::frame::value::Timestamp;
use uuid::Uuid;
use scylla::frame::response::result::Row;
use crate::db::models::whiteboard::{InputGetWhiteboardScribble, InputGetWhiteboardUpload, InputGetWhiteboardTextItem, InputGetWhiteboardBookmark};
use crate::db::models::file::ReadGetWhiteboard;
use crate::api::filemanager::{delete_all_textitems_from_whiteboard, delete_all_uploads_from_whiteboard, delete_all_scribbles_from_whiteboard, delete_all_bookmarks_from_whiteboard};

pub async fn add_user(session_arc: &Arc<Session>, user: NewUser) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "INSERT INTO fluffy_board.account (id, name, email, password, created) \
            VALUES (?, ?, ?, ?, ?);",
            (user.id, user.name, user.email, user.password, user.created),
        )
        .await?;
    Ok(())
}

pub async fn get_user_by_email(session_arc: &Arc<Session>, email: String) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.account WHERE email=?;",
            (email, ),
        )
        .await.ok()?.rows
}

pub async fn get_user_by_id(session_arc: &Arc<Session>, id: Uuid) -> Option<Vec<Row>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "SELECT * FROM fluffy_board.account WHERE id=?;",
            (id, ),
        )
        .await.ok()?.rows
}

pub async fn delete_user_by_id(session_arc: &Arc<Session>, id: Uuid) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    // FileManager
    session
        .query(
            "DELETE FROM fluffy_board.wb_directory WHERE owner=?;",
            (id, ),
        )
        .await?;
    session
        .query(
            "DELETE FROM fluffy_board.ext_whiteboard WHERE account=?;",
            (id, ),
        )
        .await?;
    let whiteboards = session
        .query(
            "SELECT FROM fluffy_board.whiteboard WHERE owner=?;",
            (id, ),
        )
        .await?.rows;
    for row in whiteboards.unwrap().into_typed::<ReadGetWhiteboard>() {
        let unwraped_row: ReadGetWhiteboard = row.unwrap();
        delete_all_scribbles_from_whiteboard(&session, InputGetWhiteboardScribble { whiteboard: unwraped_row.id, permission_id: unwraped_row.edit_id }).await;
        delete_all_uploads_from_whiteboard(&session, InputGetWhiteboardUpload { whiteboard: unwraped_row.id, permission_id: unwraped_row.edit_id }).await;
        delete_all_textitems_from_whiteboard(&session, InputGetWhiteboardTextItem { whiteboard: unwraped_row.id, permission_id: unwraped_row.edit_id }).await;
        deleta_all_bookmarks_from_whiteboard(&session, InputGetWhiteboardBookmark { whiteboard: unwraped_row.id, permission_id: unwraped_row.edit_id }).await;
    }

    session
        .query(
            "DELETE FROM fluffy_board.whiteboard WHERE owner=?;",
            (id, ),
        )
        .await?;

    // Options
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_pencil WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_highlighter WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_eraser WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_straight_line WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_figure WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // session
    //     .query(
    //         "DELETE FROM fluffy_board.toolbar_options_background WHERE owner=?;",
    //         (id, ),
    //     )
    //     .await?;
    // Account
    session
        .query(
            "DELETE FROM fluffy_board.account WHERE id=?;",
            (id, ),
        )
        .await?;
    Ok(())
}

pub async fn update_username_by_id(session_arc: &Arc<Session>, name: String,
                                   id: Uuid, email: String) -> Result<(), Box<dyn Error>> {
    let session = Arc::clone(session_arc);
    session
        .query(
            "UPDATE fluffy_board.account SET name=? WHERE id=? AND email=?;",
            (name, id, email),
        )
        .await?;
    Ok(())
}
