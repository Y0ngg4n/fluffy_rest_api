extern crate actix_web;

use std::{env};
use std::error::Error;

use actix_web::{App, HttpServer, middleware, web};
use scylla::Session;

use db::database;
use db::database_migrations;
use std::sync::Arc;

use actix::*;
use crate::api::websocket::lobby::Lobby;
use actix_cors::Cors;

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
    database_migrations::migrate(&session_arc).await.expect("Could not migrate database");
    start_webserver(Arc::clone(&session_arc));
    Ok(())
}

#[actix_web::main]
async fn start_webserver(session: Arc<Session>) -> Result<(), Box<dyn Error>> {
    let websocket_lobby_server = Lobby::default(&session).start(); //create and spin up a lobby
    // Starting Webserver
    println!("Starting Webserver");
    HttpServer::new(move || {
        let cors = Cors::default().supports_credentials().allow_any_origin().allow_any_header().allow_any_method();

        App::new()
            .wrap(cors)
            .data(Arc::clone(&session))
            .data(websocket_lobby_server.clone())
            .app_data(web::PayloadConfig::new(usize::MAX))
            .app_data(
                // Json extractor configuration for this resource.
                web::JsonConfig::default()
                    .limit(usize::MAX) // Limit request payload size
            )
// enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
// register HTTP requests handlers
            .service(web::scope("/account").configure(api::account::init_routes))
            .service(web::scope("/filemanager").configure(api::filemanager::init_routes))
            .service(web::scope("/filemanager-ext").configure(api::ext_filemanager::init_routes))
            .service(web::scope("/toolbar-options/pencil").configure(api::toolbar_options::pencil::init_routes))
            .service(web::scope("/toolbar-options/highlighter").configure(api::toolbar_options::highlighter::init_routes))
            .service(web::scope("/toolbar-options/eraser").configure(api::toolbar_options::eraser::init_routes))
            .service(web::scope("/toolbar-options/text-item").configure(api::toolbar_options::text_item::init_routes))
            .service(web::scope("/toolbar-options/straight-line").configure(api::toolbar_options::straight_line::init_routes))
            .service(web::scope("/toolbar-options/figure").configure(api::toolbar_options::figure::init_routes))
            .service(web::scope("/toolbar-options/background").configure(api::toolbar_options::background::init_routes))
            .service(web::scope("/whiteboard").configure(api::whiteboard_data::init_routes))
            .service(web::scope("/offline-whiteboard").configure(api::offline_whiteboard::init_routes))
            //         Websocket
            .service(web::scope("/ws").configure(api::websocket::start_connection::init_routes))
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await.expect("Could not start Webserver");
    Ok(())
}