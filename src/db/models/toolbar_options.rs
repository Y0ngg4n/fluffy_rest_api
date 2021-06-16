use uuid::Uuid;
use scylla::IntoTypedRows;
use scylla::macros::FromRow;
use scylla::frame::response::cql_to_rust::FromRow;
use serde::{Deserialize, Serialize};

#[derive(FromRow)]
pub struct ReadGetPencil{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdatePencil{
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdatePencil{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(FromRow)]
pub struct ReadGetHighlighter{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateHighlighter{
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateHighlighter{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64
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
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct InputUpdateStraightLine{
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}

#[derive(Serialize, Deserialize)]
pub struct NewUpdateStraightLine{
    pub owner: Uuid,
    pub color_presets: Vec<String>,
    pub stroke_width: f64
}


