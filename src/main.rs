#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::middleware::{Compress, Logger};
use actix_web::web;
use actix_web::{App, HttpServer};

mod api;
mod database;
mod models;
mod schema;
mod services;
mod utils;

fn main() -> std::io::Result<()> {
    utils::logger::init().unwrap_or_default();
    let sys = actix_rt::System::new("rut-server-rust");
    let pool = database::init_db_pool();
    let bind_host = dotenv::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8083".to_string());

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(Compress::default())
            .service(
                web::scope("/api")
                    .route("/auth/sign_up", web::post().to_async(api::account::sign_up))
                    .route("/auth/sign_in", web::post().to_async(api::account::sign_in))
                    .route("/logined", web::get().to_async(api::account::require_login)),
            )
    })
    .bind(&bind_host)
    .expect("Can not bind to host")
    .start();

    sys.run()
}
