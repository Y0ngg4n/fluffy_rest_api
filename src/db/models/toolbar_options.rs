use uuid::Uuid;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use serde::{Deserialize, Serialize};

#[derive(FromRow)]
pub struct ReadGetPencil{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub selected_color: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdatePencil{
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdatePencil{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
}

#[derive(FromRow)]
pub struct ReadGetHighlighter{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub selected_color: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateHighlighter{
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateHighlighter{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
}

#[derive(FromRow)]
pub struct ReadGetEraser{
    pub owner: Uuid,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateEraser{
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateEraser{
    pub owner: Uuid,
    pub stroke_width: f64
}

#[derive(FromRow)]
pub struct ReadGetStraightLine{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub selected_cap: i32,
    pub selected_color: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateStraightLine{
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
    pub selected_cap: i32,
}

#[derive(FromRow)]
pub struct ReadGetTextItem{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub selected_cap: i32,
    pub selected_color: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateTextItem{
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
    pub selected_cap: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateTextItem{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
    pub selected_cap: i32,
}

#[derive(FromRow)]
pub struct ReadGetFigure{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub selected_color: i32,
    pub selected_figure: i32,
    pub selected_fill: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateFigure{
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
    pub selected_figure: i32,
    pub selected_fill: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateFigure{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64,
    pub selected_color: i32,
    pub selected_figure: i32,
    pub selected_fill: i32,
}

#[derive(FromRow)]
pub struct ReadGetBackground{
    pub owner: Uuid,
    pub selected_background: i32,
    pub stroke_width: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateBackground{
    pub stroke_width: f64,
    pub selected_background: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateBackground{
    pub owner: Uuid,
    pub stroke_width: f64,
    pub selected_background: i32,
}