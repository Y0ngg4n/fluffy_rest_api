extern crate actix_web;

use std::{env, io};
use std::error::Error;

use actix_web::{App, HttpServer, middleware, web};
use futures::TryFutureExt;
use scylla::Session;

use db::database;
use std::sync::Arc;
use actix_web::http::header;
use actix_web::web::resource;
use std::time::Duration;
use std::{thread};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};
use crate::api::websocket::lobby::Lobby;

mod db;
mod api;
mod utils;
mod middlewares;

// #[actix_rt::main]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Database
    let session: Session = database::connect().await.expect("Could not connect to Database!");
    let session_arc = Arc::new(session);
    database::create_keyspace_and_tables(&session_arc).await.expect("Could not create Keyspace and \
    // // Tables!");
    start_webserver(Arc::clone(&session_arc));
    Ok(())
}

#[actix_web::main]
async fn start_webserver(session: Arc<Session>) -> Result<(), Box<dyn Error>> {
    let websocket_lobby_server = Lobby::default(&session).start(); //create and spin up a lobby
    // Starting Webserver
    println!("Starting Webserver");
    HttpServer::new(move ||
        App::new()
            .data(Arc::clone(&session))
            .data(websocket_lobby_server.clone())

// enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
// register HTTP requests handlers
            .service(web::scope("/account").configure(api::account::init_routes))
            .service(web::scope("/filemanager").configure(api::filemanager::init_routes))
            .service(web::scope("/filemanager-ext").configure(api::ext_filemanager::init_routes))
            .service(web::scope("/toolbar-options/pencil").configure(api::toolbar_options::pencil::init_routes))
            .service(web::scope("/toolbar-options/highlighter").configure(api::toolbar_options::highlighter::init_routes))
            .service(web::scope("/toolbar-options/eraser").configure(api::toolbar_options::eraser::init_routes))
            .service(web::scope("/toolbar-options/straight-line").configure(api::toolbar_options::straight_line::init_routes))
            .service(web::scope("/toolbar-options/figure").configure(api::toolbar_options::figure::init_routes))
            .service(web::scope("/toolbar-options/background").configure(api::toolbar_options::background::init_routes))
            .service(web::scope("/whiteboard").configure(api::whiteboard_data::init_routes))
    //         Websocket
            .service(web::scope("/ws").configure(api::websocket::start_connection::init_routes))
    )
        .bind("0.0.0.0:9090")?
        .run()
        .await.expect("Could not start Webserver");
    Ok(())
}