use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::websocket::websocket_types::DrawPoint;

#[derive(Serialize, Deserialize)]
pub struct InputImport {
    pub uuid: Uuid,
    pub directory: String,
    pub name: String,
    pub uploads: Vec<InputImportUpload>,
    pub texts: Vec<InputImportTextItem>,
    pub scribbles: Vec<InputImportScribble>,
}

#[derive(Serialize, Deserialize)]
pub struct InputImportUpload {
    pub uuid: Uuid,
    pub upload_type: i32,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub uint8list: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct InputImportTextItem {
    pub uuid: Uuid,
    pub stroke_width: f64,
    pub max_width: i32,
    pub max_height: i32,
    pub color: String,
    pub text: String,
    pub offset_dx: f64,
    pub offset_dy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputImportScribble {
    pub uuid: Uuid,
    pub selected_figure_type_toolbar: i32,
    pub stroke_width: f64,
    pub stroke_cap: i32,
    pub color: String,
    pub points: Vec<DrawPoint>,
    pub painting_style: i32,
    pub left_extremity: f64,
    pub right_extremity: f64,
    pub top_extremity: f64,
    pub bottom_extremity: f64
}