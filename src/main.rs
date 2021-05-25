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
async fn start_webserver(session: Arc<Session>) -> std::io::Result<()> {
    // Starting Webserver
    HttpServer::new(move ||
        App::new()
            .data(Arc::clone(&session))

// enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
// register HTTP requests handlers
            .service(web::scope("/account").configure(api::account::init_routes))
    )
        .bind("0.0.0.0:9090")?
        .run()
        .await.expect("Could not start Webserver!");
    Ok(())
}