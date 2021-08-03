use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use crate::db::toolbar_options::pencil::{get_pencil, update_pencil};
use actix_web::{get, post, Responder, web, HttpResponse};
use crate::db::models::toolbar_options::{ReadGetPencil, InputUpdatePencil, NewUpdatePencil};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    color_presets: Vec<String>,
    stroke_width: f64,
    selected_color: i32,
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_pencil(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let pencil_options = rows.into_typed::<ReadGetPencil>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                color_presets: pencil_options.color_presets,
                stroke_width: pencil_options.stroke_width,
                selected_color: pencil_options.selected_color,
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  pencil: web::Json<InputUpdatePencil>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_pencil(&session, NewUpdatePencil{
        owner: uuid,
        color_presets: pencil.color_presets.clone(),
        stroke_width: pencil.stroke_width.clone(),
        selected_color: pencil.selected_color.clone()
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}