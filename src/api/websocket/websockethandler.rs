//! Simple echo websocket server.
//! Open `http://localhost:8080/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use bytes::Bytes;
use actix_web_actors::ws::WebsocketContext;
use serde_json::{Value};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// do websocket handshake and start `MyWebSocket` actor
pub(crate) async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // println!("{:?}", r);
    let res = ws::start(WebsocketHandler::new(), &r, stream);
    // println!("{:?}", res);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub(crate) struct WebsocketHandler {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
}

impl Actor for WebsocketHandler {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketHandler {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        // process websocket messages
        // println!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // TO write something to the client do ctx.text()
            Ok(ws::Message::Text(text)) => WebsocketHandler::onTextMessage(text, ctx),
            Ok(ws::Message::Binary(bin)) => WebsocketHandler::onBinaryMessage(bin, ctx),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WebsocketHandler {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn onTextMessage(text: String, ctx: &mut <Self as Actor>::Context){
        let json: serde_json::Value = parse_json(text);
        if json["type"] == "scribble-add" {

        }
        ctx.text("Hallo zurück");
    }

    fn onBinaryMessage(bin: Bytes, ctx: &mut <Self as Actor>::Context){
        println!("Binary");
    }
}

fn parse_json(text: String) -> serde_json::Value{
    serde_json::from_str(text.as_ref()).expect("Could not parse JSON")
}

