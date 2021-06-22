use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post, get, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::auth::AuthorizationService;
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::file::{InputCreateDirectory, NewCreateDirectory, InputRenameDirectory, NewRenameDirectory, InputDeleteDirectory, NewCreateWhiteboard, InputCreateWhiteboard, InputRenameWhiteboard, NewRenameWhiteboard, InputDeleteWhiteboard, NewGetDirectory, InputGetDirectory, ReadGetDirectory, InputGetWhiteboard, NewGetWhiteboard, ReadGetWhiteboard, NewDeleteDirectory, NewDeleteWhiteboard};
use crate::db::filemanager::{create_directory, rename_directory, delete_directory, create_whiteboard, rename_whiteboard, delete_whiteboard, get_directory, get_whiteboard};
use scylla::frame::value::Timestamp;
use chrono::{Duration, Utc};
use std::error::Error;
use std::future::Future;
use async_recursion::async_recursion;
use crate::db::ext_filemanager::{get_ext_whiteboard, delete_ext_whiteboard};
use crate::api::filemanager::{parse_own_uuid, parse_dir_uuid};
use crate::db::whiteboard_data::{get_whiteboard_scribbles, get_whiteboard_upload};
use crate::db::models::whiteboard::{InputGetWhiteboardScribble, ReadGetWhiteboardScribble, InputGetWhiteboardUpload, ReadGetWhiteboardUpload};
use crate::db::websocket::websocket_types::DrawPoint;

#[derive(Serialize, Deserialize)]
pub struct ResponseGetWhiteboardScribble {
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
}

#[derive(Serialize, Deserialize)]
pub struct ResponseGetWhiteboardUpload {
    pub id: Uuid,
    pub image_data: Vec<u8>,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub upload_type: i32,
}



#[post("/scribble/get")]
pub async fn scribbles_get(auth: AuthorizationService, whiteboard: web::Json<InputGetWhiteboardScribble>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardScribble> = Vec::new();
    if let Some(rows) = get_whiteboard_scribbles(&session, whiteboard.0).await {
        for row in rows.into_typed::<ReadGetWhiteboardScribble>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(ResponseGetWhiteboardScribble {
                id: unwraped_row.id,
                bottom_extremity: unwraped_row.bottom_extremity,
                color: unwraped_row.color,
                left_extremity: unwraped_row.left_extremity,
                painting_style: unwraped_row.painting_style,
                points: unwraped_row.points,
                right_extremity: unwraped_row.right_extremity,
                selected_figure_type_toolbar: unwraped_row.selected_figure_type_toolbar,
                stroke_cap: unwraped_row.stroke_cap,
                stroke_width: unwraped_row.stroke_width,
                top_extremity: unwraped_row.top_extremity,
            });
        }
        HttpResponse::Ok().json(rows_vec)
    } else {
        HttpResponse::Ok().json(rows_vec)
    }
}

#[post("/upload/get")]
pub async fn upload_get(auth: AuthorizationService, upload: web::Json<InputGetWhiteboardUpload>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardUpload> = Vec::new();
    if let Some(rows) = get_whiteboard_upload(&session, upload.0).await {
        for row in rows.into_typed::<ReadGetWhiteboardUpload>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(ResponseGetWhiteboardUpload {
                id: unwraped_row.id,
                image_data: unwraped_row.image_data,
                offset_dx: unwraped_row.offset_dx,
                offset_dy: unwraped_row.offset_dy,
                upload_type: unwraped_row.upload_type
            });
        }
        HttpResponse::Ok().json(rows_vec)
    } else {
        HttpResponse::Ok().json(rows_vec)
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scribbles_get);
    cfg.service(upload_get);
}