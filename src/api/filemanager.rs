use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post, get, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::auth::AuthorizationService;
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::file::{InputCreateDirectory, NewCreateDirectory, InputRenameDirectory, NewRenameDirectory, InputDeleteDirectory, NewCreateWhiteboard, InputCreateWhiteboard, InputRenameWhiteboard, NewRenameWhiteboard, InputDeleteWhiteboard, NewGetDirectory, InputGetDirectory, ReadGetDirectory, InputGetWhiteboard, NewGetWhiteboard, ReadGetWhiteboard, NewDeleteDirectory};
use crate::db::filemanager::{create_directory, rename_directory, delete_directory, create_whiteboard, rename_whiteboard, delete_whiteboard, get_directory, get_whiteboard};
use scylla::frame::value::Timestamp;
use chrono::{Duration, Utc};

#[derive(Serialize, Deserialize)]
struct GetDirectoryResponse {
    pub id:  Uuid,
    pub owner: Uuid,
    pub parent: Uuid,
    pub created: i64,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
struct CreateDirectoryResponse {
    id: String,
    parent: String
}

#[post("/directory/get")]
pub async fn directory_get(auth: AuthorizationService, directory: web::Json<InputGetDirectory>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let parent_uuid;
    if directory.parent.is_empty() {
        parent_uuid = Uuid::nil();
    } else {
        parent_uuid = Uuid::parse_str(&directory.parent).unwrap();
    };
    let new_get_directory = NewGetDirectory {
        parent: parent_uuid,
        owner: uuid,
    };
    let mut rows_vec: Vec<GetDirectoryResponse> = Vec::new();
    if let Some(rows)= get_directory(&session, new_get_directory).await{
        for row in rows.into_typed::<ReadGetDirectory>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(GetDirectoryResponse{
                id: unwraped_row.id,
                owner: unwraped_row.owner,
                parent: unwraped_row.parent,
                created: unwraped_row.created.num_milliseconds(),
                filename: unwraped_row.filename,
            });
        }
        HttpResponse::Ok().json(rows_vec)
    }else{
        HttpResponse::Ok().json(rows_vec)
    }
}

#[post("/directory/create")]
pub async fn directory_create(auth: AuthorizationService, directory: web::Json<InputCreateDirectory>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let parent_uuid;
    if directory.parent.is_empty() {
        parent_uuid = Uuid::nil();
    } else {
        parent_uuid = Uuid::parse_str(&directory.parent).unwrap();
    };
    let new_uuid =  Uuid::new_v4();
    let new_directory = NewCreateDirectory {
        id: new_uuid,
        owner: uuid,
        parent: parent_uuid,
        filename: directory.filename.clone(),
        created: Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
    };
    create_directory(&session, new_directory).await.expect("Cant create Directory");
    HttpResponse::Ok().json(CreateDirectoryResponse{id: new_uuid.to_string(), parent: parent_uuid.to_string()})
}

#[post("/directory/rename")]
pub async fn directory_rename(auth: AuthorizationService, directory: web::Json<InputRenameDirectory>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let renamed_dir = NewRenameDirectory{
        id: directory.id,
        owner: uuid,
        parent: directory.parent,
        filename: directory.filename.clone()
    };
    rename_directory(&session, renamed_dir).await.expect("Cant rename Directory");
    HttpResponse::Ok().body("Directory renamed")
}

#[post("/directory/delete")]
pub async fn directory_delete(auth: AuthorizationService, directory: web::Json<InputDeleteDirectory>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let new_delete_dir = NewDeleteDirectory{
        id: directory.id,
        owner: uuid
    };
    delete_directory(&session, new_delete_dir).await.expect("Cant delete Directory");
    let new_get_delete_whitebord = NewGetWhiteboard{
        owner: uuid,
        directory: directory.id
    };
    if let Some(rows)= get_whiteboard(&session, new_get_delete_whitebord).await {
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            delete_whiteboard(&session, InputDeleteWhiteboard{
                id: unwraped_row.id,
            }).await.expect("Cant delete Whiteboard");
        }
    }
    HttpResponse::Ok().body("Directory deleted")
}

#[derive(Serialize, Deserialize)]
struct GetWhiteboardResponse {
    pub id:  Uuid,
    pub owner: Uuid,
    pub directory: Uuid,
    pub created: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct CreateWhiteboardResponse {
    id: String,
    directory: String
}

#[post("/whiteboard/get")]
pub async fn whiteboard_get(auth: AuthorizationService, whiteboard: web::Json<InputGetWhiteboard>,
                            session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let directory_uuid;
    if whiteboard.directory.is_empty() {
        directory_uuid = Uuid::nil();
    } else {
        directory_uuid = Uuid::parse_str(&whiteboard.directory).unwrap();
    };
    let new_get_whiteboard = NewGetWhiteboard {
        directory: directory_uuid,
        owner: uuid,
    };
    let mut rows_vec: Vec<GetWhiteboardResponse> = Vec::new();
    if let Some(rows)= get_whiteboard(&session, new_get_whiteboard).await{
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(GetWhiteboardResponse{
                id: unwraped_row.id,
                owner: unwraped_row.owner,
                directory: unwraped_row.directory,
                created: unwraped_row.created.num_milliseconds(),
                name: unwraped_row.name
            });
        }
        HttpResponse::Ok().json(rows_vec)
    }else{
        HttpResponse::Ok().json(rows_vec)
    }
}

#[post("/whiteboard/create")]
pub async fn whitebord_create(auth: AuthorizationService, whiteboard: web::Json<InputCreateWhiteboard>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let new_uuid =  Uuid::new_v4();
    let directory_uuid;
    if whiteboard.directory.is_empty() {
        directory_uuid = Uuid::nil();
    } else {
        directory_uuid = Uuid::parse_str(&whiteboard.directory).unwrap();
    };
    let new_whiteboard = NewCreateWhiteboard {
        id: new_uuid,
        owner: uuid,
        name: whiteboard.name.clone(),
        directory: directory_uuid,
        password: whiteboard.password.clone(),
        created:Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
    };
    create_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
    HttpResponse::Ok().json(CreateWhiteboardResponse{id: new_uuid.to_string(), directory: directory_uuid.to_string()})
}

#[post("/whiteboard/rename")]
pub async fn whiteboard_rename(auth: AuthorizationService, whiteboard: web::Json<InputRenameWhiteboard>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    let renamed_whiteboard = NewRenameWhiteboard{
        id: whiteboard.id,
        owner: uuid,
        directory: whiteboard.directory,
        name: whiteboard.name.clone(),
    };
    rename_whiteboard(&session, renamed_whiteboard).await.expect("Cant rename Directory");
    HttpResponse::Ok().body("Directory renamed")
}

#[post("/whiteboard/delete")]
pub async fn whiteboard_delete(auth: AuthorizationService, whiteboard: web::Json<InputDeleteWhiteboard>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    // let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    delete_whiteboard(&session, whiteboard.0).await.expect("Cant delete Whiteboard");
    HttpResponse::Ok().body("Whiteboard deleted")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(directory_get);
    cfg.service(directory_create);
    cfg.service(directory_rename);
    cfg.service(directory_delete);
    cfg.service(whiteboard_get);
    cfg.service(whitebord_create);
    cfg.service(whiteboard_rename);
    cfg.service(whiteboard_delete);
}