use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, web, HttpResponse};
use crate::db::models::toolbar_options::{ReadGetStraightLine, InputUpdateStraightLine, NewUpdateTextItem, ReadGetTextItem, InputUpdateTextItem};
use serde::{Deserialize, Serialize};
use crate::db::toolbar_options::text_item::{get_text_item, update_text_item};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    color_presets: Vec<String>,
    stroke_width: f64,
    selected_color: i32,
    selected_cap: i32,
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_text_item(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let text_item_options = rows.into_typed::<ReadGetTextItem>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                color_presets: text_item_options.color_presets,
                stroke_width: text_item_options.stroke_width,
                selected_color: text_item_options.selected_color,
                selected_cap: text_item_options.selected_cap
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  text_item: web::Json<InputUpdateTextItem>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_text_item(&session, NewUpdateTextItem {
        owner: uuid,
        color_presets: text_item.color_presets.clone(),
        stroke_width: text_item.stroke_width.clone(),
        selected_color: text_item.selected_color.clone(),
        selected_cap: text_item.selected_cap.clone()
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}