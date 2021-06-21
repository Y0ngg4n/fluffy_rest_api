use uuid::Uuid;
use crate::db::websocket::websocket_types::DrawPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScribbleAdd {
    pub uuid: Uuid,
    pub selected_figure_type_toolbar: i32,
    pub stroke_width: f64,
    pub stoke_cap: i32,
    pub color: String,
    pub points: Vec<DrawPoint>,
    pub painting_style: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ScribbleUpdate {
    pub uuid: Uuid,
    pub stroke_width: f64,
    pub stoke_cap: i32,
    pub color: String,
    pub points: Vec<DrawPoint>,
    pub painting_style: i32,
    pub left_extremity: f64,
    pub right_extremity: f64,
    pub top_extremity: f64,
    pub bottom_extremity: f64,
}