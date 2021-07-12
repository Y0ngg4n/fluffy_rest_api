use uuid::Uuid;
use crate::db::websocket::websocket_types::DrawPoint;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScribbleAdd {
    pub uuid: Uuid,
    pub selected_figure_type_toolbar: i32,
    pub stroke_width: f64,
    pub stroke_cap: i32,
    pub color: String,
    pub points: Vec<DrawPoint>,
    pub painting_style: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ScribbleUpdate {
    pub uuid: Uuid,
    pub stroke_width: f64,
    pub stroke_cap: i32,
    pub color: String,
    pub points: Vec<DrawPoint>,
    pub painting_style: i32,
    pub left_extremity: f64,
    pub right_extremity: f64,
    pub top_extremity: f64,
    pub bottom_extremity: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ScribbleDelete {
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UploadAdd {
    pub uuid: Uuid,
    pub upload_type: i32,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub image_data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadUpdate {
    pub uuid: Uuid,
    pub offset_dx: f64,
    pub offset_dy: f64,
}


#[derive(Serialize, Deserialize)]
pub struct UploadImageDataUpdate {
    pub uuid: Uuid,
    pub image_data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadDelete {
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct TextItemAdd {
    pub uuid: Uuid,
    pub stroke_width: f64,
    pub max_width: i32,
    pub max_height: i32,
    pub color: String,
    pub content_text: String,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub rotation: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TextItemUpdate {
    pub uuid: Uuid,
    pub stroke_width: f64,
    pub max_width: i32,
    pub max_height: i32,
    pub color: String,
    pub content_text: String,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub rotation: f64,
}

#[derive(Serialize, Deserialize)]
pub struct TextItemDelete {
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct UserMove {
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub scale: f64
}

#[derive(Serialize, Deserialize)]
pub struct UserMoveSend {
    pub uuid: Uuid,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub scale: f64
}