use crate::database::DbAddr;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures::future::result;
use futures::Future;

// "/ruts" POST
pub fn new(_db: Data<DbAddr>) -> impl Future<Item = HttpResponse, Error = Error> {
    result(Ok(HttpResponse::Ok().body("world")))
}
