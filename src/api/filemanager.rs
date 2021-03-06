use actix_web::{web, HttpResponse, post, Responder};
use serde::{Deserialize, Serialize};
use crate::middlewares::auth::AuthorizationService;
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::file::{InputCreateDirectory, NewCreateDirectory, InputRenameDirectory, NewRenameDirectory, InputDeleteDirectory, NewCreateWhiteboard, InputCreateWhiteboard, InputRenameWhiteboard, NewRenameWhiteboard, InputDeleteWhiteboard, NewGetDirectory, InputGetDirectory, ReadGetDirectory, InputGetWhiteboard, NewGetWhiteboard, ReadGetWhiteboard, NewDeleteDirectory, NewDeleteWhiteboard, NewMoveWhiteboard, InputMoveWhiteboard, InputMoveDirectory, NewMoveDirectory, NewGetDirectoryAll};
use crate::db::filemanager::{create_directory, rename_directory, delete_directory, create_whiteboard, rename_whiteboard, delete_whiteboard, get_directory, get_whiteboard, move_whiteboard, move_directory, get_directory_all};
use scylla::frame::value::Timestamp;
use chrono::{Duration, Utc};
use async_recursion::async_recursion;
use crate::db::models::ext_file::{NewGetExtWhiteboard, NewDeleteExtWhiteboard, ReadGetExtWhiteboard};
use crate::db::ext_filemanager::{get_ext_whiteboard, delete_ext_whiteboard};
use crate::db::websocket::scribble::{scribble_delete};
use crate::db::websocket::upload::{upload_delete};
use crate::db::websocket::textitem::{text_item_delete};
use crate::db::whiteboard_data::{get_whiteboard_scribbles, get_whiteboard_upload, get_whiteboard_text_item, get_whiteboard_bookmark};
use crate::db::models::whiteboard::{ReadGetWhiteboardScribble, InputGetWhiteboardScribble, InputGetWhiteboardUpload, ReadGetWhiteboardUpload, InputGetWhiteboardTextItem, ReadGetWhiteboardTextItem, InputGetWhiteboardBookmark, ReadGetWhiteboardBookmark};
use crate::api::websocket::json_messages::{ScribbleDelete, UploadDelete, TextItemDelete, BookmarkDelete};
use crate::db::websocket::bookmark::bookmark_delete;

#[derive(Serialize, Deserialize)]
struct GetDirectoryResponse {
    pub id: Uuid,
    pub owner: Uuid,
    pub parent: Uuid,
    pub created: i64,
    pub filename: String,
}

#[derive(Serialize, Deserialize)]
struct CreateDirectoryResponse {
    id: String,
    parent: String,
}

#[post("/directory/get")]
pub async fn directory_get(auth: AuthorizationService, directory: web::Json<InputGetDirectory>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let parent_uuid = parse_dir_uuid(directory.parent.clone());
    let new_get_directory = NewGetDirectory {
        parent: parent_uuid,
        owner: uuid,
    };
    let mut rows_vec: Vec<GetDirectoryResponse> = Vec::new();
    if let Some(rows) = get_directory(&session, new_get_directory).await {
        for row in rows.into_typed::<ReadGetDirectory>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(GetDirectoryResponse {
                id: unwraped_row.id,
                owner: unwraped_row.owner,
                parent: unwraped_row.parent,
                created: unwraped_row.created.num_milliseconds(),
                filename: unwraped_row.filename,
            });
        }
        HttpResponse::Ok().json(rows_vec)
    } else {
        HttpResponse::Ok().json(rows_vec)
    }
}

#[post("/directory/get-all")]
pub async fn directory_get_all(auth: AuthorizationService, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let new_get_directory = NewGetDirectoryAll {
        owner: uuid,
    };
    let mut rows_vec: Vec<GetDirectoryResponse> = Vec::new();
    if let Some(rows) = get_directory_all(&session, new_get_directory).await {
        for row in rows.into_typed::<ReadGetDirectory>() {
            let unwraped_row = row.unwrap();
            rows_vec.push(GetDirectoryResponse {
                id: unwraped_row.id,
                owner: unwraped_row.owner,
                parent: unwraped_row.parent,
                created: unwraped_row.created.num_milliseconds(),
                filename: unwraped_row.filename,
            });
        }
        HttpResponse::Ok().json(rows_vec)
    } else {
        HttpResponse::Ok().json(rows_vec)
    }
}

#[post("/directory/create")]
pub async fn directory_create(auth: AuthorizationService, directory: web::Json<InputCreateDirectory>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let parent_uuid = parse_dir_uuid(directory.parent.clone());
    let new_uuid = Uuid::new_v4();
    let new_directory = NewCreateDirectory {
        id: new_uuid,
        owner: uuid,
        parent: parent_uuid,
        filename: directory.filename.clone(),
        created: Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
    };
    create_directory(&session, new_directory).await.expect("Cant create Directory");
    HttpResponse::Ok().json(CreateDirectoryResponse { id: new_uuid.to_string(), parent: parent_uuid.to_string() })
}

#[post("/directory/move")]
pub async fn directory_move(auth: AuthorizationService, directory: web::Json<InputMoveDirectory>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let directory_uuid = parse_dir_uuid(directory.parent.clone());
    let renamed_dir = NewMoveDirectory {
        id: directory.id,
        parent: directory_uuid,
    };
    move_directory(&session, renamed_dir).await.expect("Cant move Directory");
    HttpResponse::Ok().body("Directory moved")
}

#[post("/directory/rename")]
pub async fn directory_rename(auth: AuthorizationService, directory: web::Json<InputRenameDirectory>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let renamed_dir = NewRenameDirectory {
        id: directory.id,
        filename: directory.filename.clone(),
    };
    rename_directory(&session, renamed_dir).await.expect("Cant rename Directory");
    HttpResponse::Ok().body("Directory renamed")
}

#[post("/directory/delete")]
pub async fn directory_delete(auth: AuthorizationService, directory: web::Json<InputDeleteDirectory>,
                              session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    delete_sub_directory(&session, NewGetDirectory {
        owner: uuid,
        parent: directory.id,
    }).await;
    delete_directory(&session, NewDeleteDirectory {
        id: directory.id
    }).await.expect("Could not delete directory");
    HttpResponse::Ok().body("Directory deleted")
}

#[derive(Serialize, Deserialize)]
struct GetWhiteboardResponse {
    pub id: Uuid,
    pub owner: Uuid,
    pub directory: Uuid,
    pub created: i64,
    pub name: String,
    pub view_id: Uuid,
    pub edit_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct CreateWhiteboardResponse {
    id: String,
    directory: String,
}

#[post("/whiteboard/get")]
pub async fn whiteboard_get(auth: AuthorizationService, whiteboard: web::Json<InputGetWhiteboard>,
                            session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let directory_uuid = parse_dir_uuid(whiteboard.directory.clone());
    let new_get_whiteboard = NewGetWhiteboard {
        directory: directory_uuid,
        owner: uuid,
    };
    let mut response_vec: Vec<GetWhiteboardResponse> = Vec::new();
    if let Some(rows) = get_whiteboard(&session, new_get_whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            response_vec.push(GetWhiteboardResponse {
                id: unwraped_row.id,
                owner: unwraped_row.owner,
                directory: unwraped_row.directory,
                created: unwraped_row.created.num_milliseconds(),
                name: unwraped_row.name,
                view_id: unwraped_row.view_id,
                edit_id: unwraped_row.edit_id,
            });
        }
        HttpResponse::Ok().json(response_vec)
    } else {
        HttpResponse::Ok().json(response_vec)
    }
}

#[post("/whiteboard/create")]
pub async fn whiteboard_create(auth: AuthorizationService, whiteboard: web::Json<InputCreateWhiteboard>,
                               session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let new_uuid = Uuid::new_v4();
    let directory_uuid = parse_dir_uuid(whiteboard.directory.clone());
    let new_whiteboard = NewCreateWhiteboard {
        id: new_uuid,
        owner: uuid,
        name: whiteboard.name.clone(),
        directory: directory_uuid,
        password: whiteboard.password.clone(),
        created: Timestamp(Duration::milliseconds(Utc::now().timestamp_millis())),
        view_id: Uuid::new_v4(),
        edit_id: Uuid::new_v4(),
    };
    create_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
    HttpResponse::Ok().json(CreateWhiteboardResponse { id: new_uuid.to_string(), directory: directory_uuid.to_string() })
}

#[post("/whiteboard/rename")]
pub async fn whiteboard_rename(auth: AuthorizationService, whiteboard: web::Json<InputRenameWhiteboard>,
                               session: web::Data<Arc<Session>>) -> impl Responder {
    let renamed_whiteboard = NewRenameWhiteboard {
        id: whiteboard.id,
        name: whiteboard.name.clone(),
    };
    rename_whiteboard(&session, renamed_whiteboard).await.expect("Cant rename Whiteboard");
    HttpResponse::Ok().body("Directory renamed")
}

#[post("/whiteboard/move")]
pub async fn whiteboard_move(auth: AuthorizationService, whiteboard: web::Json<InputMoveWhiteboard>,
                               session: web::Data<Arc<Session>>) -> impl Responder {
    let directory_uuid = parse_dir_uuid(whiteboard.directory.clone());
    let renamed_whiteboard = NewMoveWhiteboard {
        id: whiteboard.id,
        directory: directory_uuid.clone(),
    };
    move_whiteboard(&session, renamed_whiteboard).await.expect("Cant move Whiteboard");
    HttpResponse::Ok().body("Whiteboard moved")
}

#[post("/whiteboard/delete")]
pub async fn whiteboard_delete(auth: AuthorizationService, whiteboard: web::Json<InputDeleteWhiteboard>,
                               session: web::Data<Arc<Session>>) -> impl Responder {
    delete_whiteboard(&session, NewDeleteWhiteboard {
        id: whiteboard.id
    }).await.expect("Cant delete Whiteboard");
    delete_all_scribbles_from_whiteboard(&session, InputGetWhiteboardScribble{whiteboard: whiteboard.id, permission_id: whiteboard.id}).await;
    delete_all_uploads_from_whiteboard(&session, InputGetWhiteboardUpload{whiteboard: whiteboard.id, permission_id: whiteboard.id}).await;
    delete_all_textitems_from_whiteboard(&session, InputGetWhiteboardTextItem{whiteboard: whiteboard.id, permission_id: whiteboard.id}).await;
    HttpResponse::Ok().body("Whiteboard deleted")
}

pub fn parse_own_uuid(auth: AuthorizationService) -> Uuid {
    Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap()
}

pub fn parse_dir_uuid(directory: String) -> Uuid {
    let mut uuid: Uuid = Uuid::nil();
    if !directory.is_empty() {
        uuid = Uuid::parse_str(&directory).unwrap();
    }
    uuid
}

// TODO: Add Deletion of Scribbles Uploads and TextItems
#[async_recursion]
async fn delete_sub_directory(session: &Arc<Session>, new_get_delete_directory: NewGetDirectory) {
    if let Some(rows) = get_directory(&session, new_get_delete_directory).await {
        for row in rows.into_typed::<ReadGetDirectory>() {
            let unwraped_row = row.unwrap();
            delete_sub_directory(&session, NewGetDirectory {
                parent: unwraped_row.id,
                owner: new_get_delete_directory.owner,
            }).await;
            delete_directory(&session, NewDeleteDirectory {
                id: unwraped_row.id,
            }).await.expect("Could not delete subdir");
        }
    }
    if let Some(rows) = get_ext_whiteboard(&session, NewGetExtWhiteboard {
        directory: new_get_delete_directory.parent,
        account: new_get_delete_directory.owner,
    }).await {
        for row in rows.into_typed::<ReadGetExtWhiteboard>() {
            let unwraped_row = row.unwrap();
            delete_ext_whiteboard(&session, NewDeleteExtWhiteboard {
                id: unwraped_row.id,
            }).await.expect("Cant delete ext whiteboard");
        }
    }
    if let Some(rows) = get_whiteboard(&session, NewGetWhiteboard {
        directory: new_get_delete_directory.parent,
        owner: new_get_delete_directory.owner,
    }).await {
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            delete_whiteboard(&session, NewDeleteWhiteboard {
                id: unwraped_row.id,
            }).await.expect("Cant delete sub whiteboard");
            // TODO: Change to Batch statements
            delete_all_scribbles_from_whiteboard(&session, InputGetWhiteboardScribble{ whiteboard: unwraped_row.id, permission_id: unwraped_row.id}).await;
            delete_all_uploads_from_whiteboard(&session, InputGetWhiteboardUpload { whiteboard: unwraped_row.id, permission_id: unwraped_row.id}).await;
            delete_all_textitems_from_whiteboard(&session, InputGetWhiteboardTextItem{ whiteboard: unwraped_row.id, permission_id: unwraped_row.id}).await;
            delete_all_bookmarks_from_whiteboard(&session, InputGetWhiteboardBookmark{whiteboard: unwraped_row.id, permission_id: unwraped_row.id}).await;
        }
    }
}

pub async fn delete_all_scribbles_from_whiteboard(session: &Arc<Session>, whiteboard: InputGetWhiteboardScribble){
    if let Some(rows) = get_whiteboard_scribbles(&session, whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboardScribble>() {
            let unwraped_row = row.unwrap();
            scribble_delete(session.clone(), ScribbleDelete{uuid: unwraped_row.id}).await;
        }
    }
}

pub async fn delete_all_uploads_from_whiteboard(session: &Arc<Session>, whiteboard: InputGetWhiteboardUpload){
    if let Some(rows) = get_whiteboard_upload(&session, whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboardUpload>() {
            let unwraped_row = row.unwrap();
            upload_delete(session.clone(), UploadDelete{uuid: unwraped_row.id}).await;
        }
    }
}

pub async fn delete_all_textitems_from_whiteboard(session: &Arc<Session>, whiteboard: InputGetWhiteboardTextItem){
    if let Some(rows) = get_whiteboard_text_item(&session, whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboardTextItem>() {
            let unwraped_row = row.unwrap();
            text_item_delete(session.clone(), TextItemDelete{uuid: unwraped_row.id}).await;
        }
    }
}

pub async fn delete_all_bookmarks_from_whiteboard(session: &Arc<Session>, whiteboard: InputGetWhiteboardBookmark){
    if let Some(rows) = get_whiteboard_bookmark(&session, whiteboard).await {
        for row in rows.into_typed::<ReadGetWhiteboardBookmark>() {
            let unwraped_row = row.unwrap();
            bookmark_delete(session.clone(), BookmarkDelete{uuid: unwraped_row.id}).await;
        }
    }
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(directory_get);
    cfg.service(directory_get_all);
    cfg.service(directory_create);
    cfg.service(directory_rename);
    cfg.service(directory_move);
    cfg.service(directory_delete);
    cfg.service(whiteboard_get);
    cfg.service(whiteboard_create);
    cfg.service(whiteboard_rename);
    cfg.service(whiteboard_move);
    cfg.service(whiteboard_delete);
}