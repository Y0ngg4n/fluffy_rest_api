use std::sync::Arc;
use scylla::Session;
use crate::db::models::user::NewUser;
use std::error::Error;
use scylla::frame::response::result::Row;
use uuid::Uuid;
use crate::db::models::toolbar_options::NewUpdatePencil;
use crate::api::websocket::json_messages::{ScribbleAdd, ScribbleUpdate, ScribbleDelete};

pub async fn upload_add(session: Arc<Session>, scribble: ScribbleAdd, whiteboard: Uuid) {
    session
        .query(
            "INSERT INTO fluffy_board.wb_scribble (id, whiteboard, selected_figure_type_toolbar, stroke_width, stroke_cap, painting_style, color, points, left_extremity, right_extremity, top_extremity, bottom_extremity) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);",
            (scribble.uuid, whiteboard, scribble.selected_figure_type_toolbar, scribble.stroke_width, scribble.stroke_cap, scribble.painting_style, scribble.color, scribble.points, 0.0, 0.0, 0.0, 0.0)
        ).await.expect("Could not insert scribble");
}

pub async fn upload_update(session: Arc<Session>, scribble: ScribbleUpdate) {
    session
        .query(
            "UPDATE fluffy_board.wb_scribble SET stroke_width=?, stroke_cap=?, painting_style=?, color=?, points=?, left_extremity=?, right_extremity=?, top_extremity=?, bottom_extremity=? WHERE id=?",
            (scribble.stroke_width, scribble.stroke_cap, scribble.painting_style, scribble.color, scribble.points, scribble.left_extremity, scribble.right_extremity, scribble.top_extremity, scribble.bottom_extremity, scribble.uuid)
        ).await.expect("Could not update scribble");
}

pub async fn upload_delete(session: Arc<Session>, scribble: ScribbleDelete) {
    session
        .query(
            "DELETE FROM fluffy_board.wb_scribble WHERE id=?",
            (scribble.uuid,)
        ).await.expect("Could not delete scribble");
}

