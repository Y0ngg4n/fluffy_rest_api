use uuid::Uuid;
use crate::db::websocket::websocket_types::DrawPoint;
use serde::{Deserialize, Serialize};
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;

#[derive(Serialize, Deserialize)]
pub struct InputGetWhiteboardScribble{
    pub whiteboard: Uuid,
}

#[derive(FromRow)]
pub struct ReadGetWhiteboardScribble {
    pub id: Uuid,
    pub bottom_extremity: f64,
    pub color: String,
    pub left_extremity: f64,
    pub painting_style: i32,
    pub points: Vec<DrawPoint>,
    pub right_extremity: f64,
    pub selected_figure_type_toolbar: i32,
    pub stroke_cap: i32,
    pub stroke_width: f64,
    pub top_extremity: f64,
    pub whiteboard: Uuid
}