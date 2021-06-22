use actix::{Addr, Actor, StreamHandler};
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest, web};
use actix_web_actors::ws;
use uuid::Uuid;
use crate::api::websocket::ws::WsConn;
use crate::api::websocket::lobby::Lobby;
use crate::api::websocket::websocket_tools;
use scylla::Session;
use std::sync::Arc;

use actix_web::error::PayloadError;
use ws::{handshake, WebsocketContext};
use actix_http::ws::{Codec, Message, ProtocolError};
use bytes::Bytes;
use actix::prelude::Stream;

// Code from https://github.com/antholeole/actix-sockets.git
// Thank you soooo much :)

#[get("/{whiteboard}/{jwt_token}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    Path((whiteboard, jwt_token)): Path<(Uuid, String)>,
    srv: Data<Addr<Lobby>>,
    session: web::Data<Arc<Session>>
) -> Result<HttpResponse, Error> {
    let auth_result = websocket_tools::check_auth(jwt_token.as_str());
    if auth_result.authenticated {
        let ws = WsConn::new(
            whiteboard,
            auth_result.uuid,
            srv.get_ref().clone(),
        );
        let resp = start_with_codec(ws, &req, stream, Codec::new().max_size(usize::MAX))?;
        Ok(resp)
    } else {
        Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

fn start_with_codec<A, S>(actor: A, req: &HttpRequest, stream: S, codec: Codec) -> Result<HttpResponse, Error>
    where
        A: Actor<Context = WebsocketContext<A>>
        + StreamHandler<Result<Message, ProtocolError>>,
        S: Stream<Item = Result<Bytes, PayloadError>> + 'static,
{
    let mut res = handshake(req)?;
    Ok(res.streaming(WebsocketContext::with_codec(actor, stream, codec)))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(start_connection);
}