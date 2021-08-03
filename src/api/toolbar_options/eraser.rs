use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, web, HttpResponse};
use crate::db::models::toolbar_options::{ReadGetEraser, InputUpdateEraser, NewUpdateEraser};
use serde::{Deserialize, Serialize};
use crate::db::toolbar_options::eraser::{get_eraser, update_eraser};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    stroke_width: f64,
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_eraser(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let eraser_options = rows.into_typed::<ReadGetEraser>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                stroke_width: eraser_options.stroke_width
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  highlighter: web::Json<InputUpdateEraser>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_eraser(&session, NewUpdateEraser{
        owner: uuid,
        stroke_width: highlighter.stroke_width.clone(),
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}