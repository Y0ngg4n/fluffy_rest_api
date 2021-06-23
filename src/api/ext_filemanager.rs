use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post, get, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::auth::AuthorizationService;
use crate::db::models::file::{InputGetDirectory, ReadGetWhiteboard};
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::ext_file::{InputCreateExtWhiteboard, NewCreateExtWhiteboard, ReadGetExtWhiteboard, NewGetOtherWhiteboard, NewDeleteExtWhiteboard, InputDeleteExtWhiteboard, InputGetExtWhiteboard, NewGetExtWhiteboard};
use chrono::format::Numeric::Timestamp;
use chrono::Duration;
use crate::api::filemanager::{parse_dir_uuid, parse_own_uuid};
use crate::db::ext_filemanager::{create_ext_whiteboard, delete_ext_whiteboard, get_ext_whiteboard};
use crate::db::ext_filemanager::get_other_whiteboard;

#[derive(Serialize, Deserialize)]
struct GetExtWhiteboardResponse {
    pub id: Uuid,
    pub account: Uuid,
    pub directory: Uuid,
    pub name: String,
    pub edit: bool,
    pub original: Uuid,
    pub permission_id: Uuid,
}

#[derive(Serialize, Deserialize)]
struct CreateWhiteboardExtResponse {
    id: String,
    directory: String,
}

#[post("/whiteboard/get")]
pub async fn whiteboard_ext_get(auth: AuthorizationService, whiteboard: web::Json<InputGetExtWhiteboard>,
                                session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let directory_uuid = parse_dir_uuid(whiteboard.directory.clone());
    let new_get_whiteboard = NewGetExtWhiteboard {
        account: uuid,
        directory: directory_uuid,
    };
    let mut response_vec: Vec<GetExtWhiteboardResponse> = Vec::new();
    if let Some(rows) = get_ext_whiteboard(&session, new_get_whiteboard).await {
        for row in rows.into_typed::<ReadGetExtWhiteboard>() {
            let unwraped_row = row.unwrap();
            response_vec.push(GetExtWhiteboardResponse {
                id: unwraped_row.id,
                account: unwraped_row.account,
                directory: unwraped_row.directory,
                name: unwraped_row.name,
                edit: unwraped_row.edit,
                original: unwraped_row.original,
                permission_id: unwraped_row.permission_id
            });
        }
        HttpResponse::Ok().json(response_vec)
    } else {
        HttpResponse::Ok().json(response_vec)
    }
}

#[post("/whiteboard/create")]
pub async fn whiteboard_ext_create(auth: AuthorizationService, whiteboard: web::Json<InputCreateExtWhiteboard>,
                                   session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = parse_own_uuid(auth);
    let directory_uuid = parse_dir_uuid(whiteboard.directory.clone());
    let board_uuid = whiteboard.id;
    let new_uuid = Uuid::new_v4();
    if let Some(rows) = get_other_whiteboard(&session, NewGetOtherWhiteboard {
        id: board_uuid,
    }).await {
        for row in rows.into_typed::<ReadGetWhiteboard>() {
            let unwraped_row = row.unwrap();
            println!("{}", unwraped_row.edit_id);
            println!("{}", whiteboard.permission_id);
            if unwraped_row.edit_id == whiteboard.permission_id {
                println!("Edit");
                let new_whiteboard = NewCreateExtWhiteboard {
                    id: new_uuid,
                    account: uuid,
                    name: unwraped_row.name.clone(),
                    directory: directory_uuid,
                    original: board_uuid,
                    edit: true,
                    permission_id: whiteboard.permission_id
                };
                create_ext_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
                // HttpResponse::Ok().body("")
            } else if unwraped_row.view_id == whiteboard.permission_id {
                println!("View");
                let new_whiteboard = NewCreateExtWhiteboard {
                    id: new_uuid,
                    account: uuid,
                    name: unwraped_row.name.clone(),
                    directory: directory_uuid,
                    original: board_uuid,
                    edit: false,
                    permission_id: whiteboard.permission_id
                };
                create_ext_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
            }
        }
        HttpResponse::Ok().json(CreateWhiteboardExtResponse { id: new_uuid.to_string(), directory: directory_uuid.to_string() })
    } else {
        HttpResponse::BadRequest().body("No such board")
    }
}

#[post("/whiteboard/delete")]
pub async fn whiteboard_ext_delete(auth: AuthorizationService, whiteboard: web::Json<InputDeleteExtWhiteboard>,
                                   session: web::Data<Arc<Session>>) -> impl Responder {
    delete_ext_whiteboard(&session, NewDeleteExtWhiteboard {
        id: whiteboard.id
    }).await.expect("Cant delete Whiteboard");
    HttpResponse::Ok().body("Whiteboard deleted")
}

pub fn init_routes(cfg: &mut web::ServiceConfig)
{
    cfg.service(whiteboard_ext_get);
    cfg.service(whiteboard_ext_create);
    cfg.service(whiteboard_ext_delete);
}
