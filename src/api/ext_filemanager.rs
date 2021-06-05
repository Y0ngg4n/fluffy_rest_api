use actix_web::web::{Data, Json, Path};
use actix_web::{web, HttpResponse, post, get, Responder};
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::auth::AuthorizationService;
use crate::db::models::file::{InputGetDirectory, ReadGetWhiteboard};
use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use uuid::Uuid;
use crate::db::models::ext_file::{InputCreateExtWhiteboard, NewCreateExtWhiteboard, ReadGetExtWhiteboard, NewGetOtherWhiteboard, NewDeleteExtWhiteboard, InputDeleteExtWhiteboard};
use chrono::format::Numeric::Timestamp;
use chrono::Duration;
use crate::api::filemanager::{parse_dir_uuid, parse_own_uuid};
use crate::db::ext_filemanager::{create_ext_whiteboard, delete_ext_whiteboard};
use crate::db::ext_filemanager::get_other_whiteboard;

#[derive(Serialize, Deserialize)]
struct CreateWhiteboardExtResponse {
    id: String,
    directory: String,
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
            if unwraped_row.edit_id == whiteboard.permission_id {
                let new_whiteboard = NewCreateExtWhiteboard {
                    id: new_uuid,
                    account: uuid,
                    name: unwraped_row.name.clone(),
                    directory: directory_uuid,
                    data: Uuid::new_v4(),
                    edit: true,
                };
                create_ext_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
                // HttpResponse::Ok().body("")
            } else if unwraped_row.view_id == whiteboard.permission_id {
                let new_whiteboard = NewCreateExtWhiteboard {
                    id: new_uuid,
                    account: uuid,
                    name: unwraped_row.name.clone(),
                    directory: directory_uuid,
                    data: Uuid::new_v4(),
                    edit: false,
                };
                create_ext_whiteboard(&session, new_whiteboard).await.expect("Cant create Whiteboard");
            }
        }
        HttpResponse::Ok().json(CreateWhiteboardExtResponse { id: new_uuid.to_string(), directory: directory_uuid.to_string() })
    }else{
        HttpResponse::BadRequest().body("No such board")
    }
}

#[post("/whiteboard/delete")]
pub async fn whiteboard_ext_delete(auth: AuthorizationService, whiteboard: web::Json<InputDeleteExtWhiteboard>,
                               session: web::Data<Arc<Session>>) -> impl Responder {
    delete_ext_whiteboard(&session, NewDeleteExtWhiteboard{
        id: whiteboard.id
    }).await.expect("Cant delete Whiteboard");
    HttpResponse::Ok().body("Whiteboard deleted")
}

pub fn init_routes(cfg: &mut web::ServiceConfig)
{
    cfg.service(whiteboard_ext_create);
    cfg.service(whiteboard_ext_delete);
}
