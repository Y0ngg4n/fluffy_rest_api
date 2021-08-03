use actix_web::{web, HttpResponse, post, Responder};
use serde::{Deserialize, Serialize};
use crate::middlewares::auth::AuthorizationService;
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::whiteboard_data::{get_whiteboard_scribbles, get_whiteboard_upload, get_whiteboard_by_id, get_whiteboard_text_item, get_whiteboard_bookmark};
use crate::db::models::whiteboard::{InputGetWhiteboardScribble, ReadGetWhiteboardScribble, InputGetWhiteboardUpload, ReadGetWhiteboardUpload, InputGetWhiteboardTextItem, ReadGetWhiteboardTextItem, InputGetWhiteboardBookmark, ReadGetWhiteboardBookmark};
use crate::db::websocket::websocket_types::DrawPoint;
use crate::db::models::file::ReadGetWhiteboard;

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

#[derive(Serialize, Deserialize)]
pub struct ResponseGetWhiteboardTextItem{
    pub id: Uuid,
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
pub struct ResponseGetWhiteboardBookmark{
    pub id: Uuid,
    pub name: String,
    pub offset_dx: f64,
    pub offset_dy: f64,
    pub scale: f64,
}

#[post("/scribble/get")]
pub async fn scribbles_get(auth: AuthorizationService, whiteboard: web::Json<InputGetWhiteboardScribble>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardScribble> = Vec::new();
    if check_permission(whiteboard.whiteboard, whiteboard.permission_id, &session).await {
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
    } else {
        HttpResponse::Forbidden().body("Wrong Permission")
    }
}

#[post("/upload/get")]
pub async fn upload_get(auth: AuthorizationService, upload: web::Json<InputGetWhiteboardUpload>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardUpload> = Vec::new();
    if check_permission(upload.whiteboard, upload.permission_id, &session).await {
        if let Some(rows) = get_whiteboard_upload(&session, upload.0).await {
            for row in rows.into_typed::<ReadGetWhiteboardUpload>() {
                let unwraped_row = row.unwrap();
                rows_vec.push(ResponseGetWhiteboardUpload {
                    id: unwraped_row.id,
                    image_data: unwraped_row.image_data,
                    offset_dx: unwraped_row.offset_dx,
                    offset_dy: unwraped_row.offset_dy,
                    upload_type: unwraped_row.upload_type,
                });
            }
            HttpResponse::Ok().json(rows_vec)
        } else {
            HttpResponse::Ok().json(rows_vec)
        }
    } else {
        HttpResponse::Forbidden().body("Wrong Permission")
    }
}

#[post("/textitem/get")]
pub async fn text_item_get(auth: AuthorizationService, textitem: web::Json<InputGetWhiteboardTextItem>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardTextItem> = Vec::new();
    if check_permission(textitem.whiteboard, textitem.permission_id, &session).await {
        if let Some(rows) = get_whiteboard_text_item(&session, textitem.0).await {
            for row in rows.into_typed::<ReadGetWhiteboardTextItem>() {
                let unwraped_row = row.unwrap();
                rows_vec.push(ResponseGetWhiteboardTextItem {
                    id: unwraped_row.id,
                    stroke_width: unwraped_row.stroke_width,
                    max_width: unwraped_row.max_width,
                    max_height: unwraped_row.max_height,
                    color: unwraped_row.color,
                    content_text: unwraped_row.content_text,
                    offset_dx: unwraped_row.offset_dx,
                    offset_dy: unwraped_row.offset_dy,
                    rotation: unwraped_row.rotation
                });
            }
            HttpResponse::Ok().json(rows_vec)
        } else {
            HttpResponse::Ok().json(rows_vec)
        }
    } else {
        HttpResponse::Forbidden().body("Wrong Permission")
    }
}

#[post("/bookmark/get")]
pub async fn bookmark_get(auth: AuthorizationService, bookmark: web::Json<InputGetWhiteboardBookmark>, session: web::Data<Arc<Session>>) -> impl Responder {
    let mut rows_vec: Vec<ResponseGetWhiteboardBookmark> = Vec::new();
    if check_permission(bookmark.whiteboard, bookmark.permission_id, &session).await {
        if let Some(rows) = get_whiteboard_bookmark(&session, bookmark.0).await {
            for row in rows.into_typed::<ReadGetWhiteboardBookmark>() {
                let unwraped_row = row.unwrap();
                rows_vec.push(ResponseGetWhiteboardBookmark{
                    id: unwraped_row.id,
                    name: unwraped_row.name,
                    offset_dx: unwraped_row.offset_dx,
                    offset_dy: unwraped_row.offset_dy,
                    scale: unwraped_row.scale
                });
            }
            HttpResponse::Ok().json(rows_vec)
        } else {
            HttpResponse::Ok().json(rows_vec)
        }
    } else {
        HttpResponse::Forbidden().body("Wrong Permission")
    }
}

pub async fn check_permission(whiteboard: Uuid, permission_id: Uuid, session: &Arc<Session>) -> bool {
    let mut auth = false;
    if let Some(rows) = get_whiteboard_by_id(&session, whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            if unwraped_row.view_id == permission_id || unwraped_row.edit_id == permission_id {
                auth = true;
                break;
            }
        }
    }
    return auth;
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(scribbles_get);
    cfg.service(upload_get);
    cfg.service(text_item_get);
    cfg.service(bookmark_get);
}