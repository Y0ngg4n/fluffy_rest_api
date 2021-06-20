use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest, web};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::api::websocket::ws::WsConn;
use crate::api::websocket::lobby::Lobby;
use crate::api::websocket::websocket_tools;

// Code from https://github.com/antholeole/actix-sockets.git
// Thank you soooo much :)

#[get("/{whiteboard}/{jwt_token}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path((whiteboard, jwt_token)): Path<(Uuid, String)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("{}", jwt_token);
    let auth_result = websocket_tools::check_auth(jwt_token.as_str());
    if auth_result.authenticated {
        let ws = WsConn::new(
            whiteboard,
            auth_result.uuid,
            srv.get_ref().clone(),
        );
        let resp = ws::start(ws, &req, stream)?;
        Ok(resp)
    } else {
        Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(start_connection);
}