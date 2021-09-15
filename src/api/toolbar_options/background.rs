use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::models::toolbar_options::{NewUpdateBackground, InputUpdateBackground, ReadGetBackground};
use crate::db::toolbar_options::background::{get_background, update_background};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    stroke_width: f64,
    selected_background: i32,
    color_presets: Vec<String>
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_background(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let background_options = rows.into_typed::<ReadGetBackground>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                stroke_width: background_options.stroke_width,
                selected_background: background_options.selected_background,
                color_presets: background_options.color_presets,
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  background: web::Json<InputUpdateBackground>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_background(&session, NewUpdateBackground{
        owner: uuid,
        stroke_width: background.stroke_width.clone(),
        color_presets: background.color_presets.clone(),
        selected_background: background.selected_background.clone()
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}