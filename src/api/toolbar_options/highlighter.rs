use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::db::models::user::LoginUser;
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, HttpRequest, web, HttpResponse};
use crate::db::models::toolbar_options::{InputUpdateHighlighter, NewUpdateHighlighter, ReadGetHighlighter};
use serde::{Deserialize, Serialize};
use crate::db::toolbar_options::highlighter::{get_highlighter, update_highlighter};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    color_presets: Vec<String>,
    stroke_width: f64,
    selected_color: i32
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_highlighter(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let highlighter_options = rows.into_typed::<ReadGetHighlighter>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                color_presets: highlighter_options.color_presets,
                stroke_width: highlighter_options.stroke_width,
                selected_color: highlighter_options.selected_color
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  highlighter: web::Json<InputUpdateHighlighter>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_highlighter(&session, NewUpdateHighlighter{
        owner: uuid,
        color_presets: highlighter.color_presets.clone(),
        stroke_width: highlighter.stroke_width.clone(),
        selected_color: highlighter.selected_color.clone()
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}