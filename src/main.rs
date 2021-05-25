extern crate actix_web;

use std::{env, io};
use std::error::Error;

use actix_web::{App, HttpServer, middleware, web};
use futures::TryFutureExt;
use scylla::Session;

use db::database;

mod db;
mod api;

// #[actix_rt::main]
#[tokio::main]
async fn main() ->  Result<(), Box<dyn Error>>{
    dotenv::dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // Database
    let session: Session = database::connect().await.expect("Could not connect to Database!");
    database::create_keyspace_and_tables(&session).await.expect("Could not create Keyspace and \
    Tables!");
    start_webserver(&session);

    Ok(())
}
#[actix_web::main]
async fn start_webserver(session: &'static Session) -> std::io::Result<()> {
    // Starting Webserver
    HttpServer::new(|| {
        App::new()
            .data(session)
// enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
// register HTTP requests handlers
//             .service(whiteboard::create)
            .service(api::account::get_users)
            .service(api::account::get_user_by_id)
            .service(api::account::add_user)
            .service(api::account::delete_user)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await.expect("Could not start Webserver!");
    Ok(())
}
