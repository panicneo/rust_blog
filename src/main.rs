#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::{resource, scope};
use actix_web::{web, App, HttpServer};

mod api;
mod database;
mod logger;
mod schema;

fn main() -> std::io::Result<()> {
    logger::init().unwrap_or_default();

    let sys = actix_rt::System::new("rut-server-rust");
    let addr = database::init_db_pool();
    let bind_host = dotenv::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8083".to_string());

    HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .wrap(Logger::default())
            .wrap(Cors::default())
            .service(
                scope("/api").service(
                    resource("hello")
                        .route(web::get().to_async(api::account::new))
                        .route(web::post().to_async(api::account::new)),
                ),
            )
    })
    .bind(&bind_host)
    .expect("Can not bind to host")
    .start();

    sys.run()
}
