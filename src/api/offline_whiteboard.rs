use crate::middlewares::auth::AuthorizationService;
use actix_web::{web, Responder, get, post, HttpResponse};
use scylla::Session;
use std::sync::Arc;
use crate::db::models::offline::{InputImport, InputImportScribble, InputImportTextItem, InputImportUpload};
use crate::db::offline_whiteboard::{import_scribbles, import_uploads, import_textitems};

#[post("/import")]
pub async fn import(auth: AuthorizationService, import: web::Json<InputImport>, session: web::Data<Arc<Session>>) -> impl Responder {
    println!("{}", &import.uuid);
    import_scribbles(&session, &import.scribbles, &import.uuid).await.expect("Could not import Scribbles");
    import_uploads(&session, &import.uploads, &import.uuid).await.expect("Could not import Uploads");
    import_textitems(&session, &import.texts, &import.uuid).await.expect("Could not import TextItems");
    HttpResponse::Ok().body("Success")
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(import);
}