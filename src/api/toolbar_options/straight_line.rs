use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::db::models::user::LoginUser;
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, HttpRequest, web, HttpResponse};
use crate::db::models::toolbar_options::{ReadGetStraightLine, InputUpdateStraightLine, NewUpdateStraightLine};
use serde::{Deserialize, Serialize};
use crate::db::toolbar_options::straight_line::{get_straight_line, update_straight_line};

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
    if let Some(rows) = get_straight_line(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let straight_line_options = rows.into_typed::<ReadGetStraightLine>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                color_presets: straight_line_options.color_presets,
                stroke_width: straight_line_options.stroke_width,
                selected_color: straight_line_options.selected_color,
                selected_cap: straight_line_options.selected_cap
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  straight_line: web::Json<InputUpdateStraightLine>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_straight_line(&session, NewUpdateStraightLine{
        owner: uuid,
        color_presets: straight_line.color_presets.clone(),
        stroke_width: straight_line.stroke_width.clone(),
        selected_color: straight_line.selected_color.clone(),
        selected_cap: straight_line.selected_cap.clone()
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}