use std::sync::Arc;
use scylla::{Session, IntoTypedRows};
use crate::middlewares::auth::AuthorizationService;
use uuid::Uuid;
use actix_web::{get, post, Responder, web, HttpResponse};
use crate::db::models::toolbar_options::{ReadGetFigure, InputUpdateFigure, NewUpdateFigure};
use serde::{Deserialize, Serialize};
use crate::db::toolbar_options::figure::{update_figure, get_figure};

#[derive(Serialize, Deserialize)]
struct GetResponse {
    color_presets: Vec<String>,
    stroke_width: f64,
    selected_color: i32,
    selected_figure: i32,
    selected_fill: i32,
}

#[get("/get")]
pub async fn get(auth: AuthorizationService,  session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    if let Some(rows) = get_figure(&session, uuid.clone()).await {
        if rows.is_empty() {
            HttpResponse::NoContent().body("")
        }else{
            let figure_options = rows.into_typed::<ReadGetFigure>().next().unwrap().unwrap();
            HttpResponse::Ok().json(GetResponse{
                color_presets: figure_options.color_presets,
                stroke_width: figure_options.stroke_width,
                selected_color: figure_options.selected_color,
                selected_figure: figure_options.selected_figure,
                selected_fill: figure_options.selected_fill
            })
        }
    }else{
        HttpResponse::NoContent().body("")
    }
}

#[post("/update")]
pub async fn update(auth: AuthorizationService,  figure: web::Json<InputUpdateFigure>, session: web::Data<Arc<Session>>) -> impl Responder {
    let uuid = Uuid::parse_str(auth.token.claims.sub.as_str()).unwrap();
    update_figure(&session, NewUpdateFigure{
        owner: uuid,
        color_presets: figure.color_presets.clone(),
        stroke_width: figure.stroke_width.clone(),
        selected_color: figure.selected_color.clone(),
        selected_figure: figure.selected_figure.clone(),
        selected_fill: figure.selected_fill.clone(),
    }).await.expect("Not updated ");
    HttpResponse::Ok().body("Updated")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(update);
}