use crate::database::DbAddr;
use crate::models::account::AccountReg;
use actix_web::error::ResponseError;
use actix_web::web::{Data, Json};
use actix_web::{Error, HttpResponse};
use futures::future::ok;
use futures::Future;

// 账户注册
pub fn sign_up(
    db: Data<DbAddr>,
    req: Json<AccountReg>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let req = req.into_inner();
    db.send(req).from_err().and_then(|res| match res {
        Ok(account) => ok(HttpResponse::Ok().json(account)),
        Err(err) => ok(err.error_response()),
    })
}
