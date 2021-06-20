use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest, web};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::api::websocket::ws::WsConn;
use crate::api::websocket::lobby::Lobby;

// Code from https://github.com/antholeole/actix-sockets.git
// Thank you soooo much :)

#[get("/{whiteboard}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path(whiteboard): Path<Uuid>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConn::new(
        whiteboard,
        srv.get_ref().clone(),
    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(start_connection);
}