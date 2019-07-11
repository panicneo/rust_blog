use super::model::{AccountSignIn, AccountSignInResp, AccountSignUp};
use super::permission;
use crate::database::DbAddr;
use crate::utils::crypto::jwt;
use actix_web::error::ResponseError;
use actix_web::web::{Data, Json};
use actix_web::{Error, HttpResponse};
use futures::future::ok;
use futures::Future;

// 账户注册
pub fn sign_up(
    db: Data<DbAddr>,
    req: Json<AccountSignUp>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let req = req.into_inner();
    db.send(req).from_err().and_then(|res| match res {
        Ok(account) => ok(HttpResponse::Ok().json(account)),
        Err(e) => ok(e.error_response()),
    })
}

pub fn sign_in(
    db: Data<DbAddr>,
    req: Json<AccountSignIn>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let req = req.into_inner();
    db.send(req).from_err().and_then(|resp| match resp {
        Ok(account) => {
            let resp = AccountSignInResp {
                token: jwt::encode(&account.id)?,
                account,
            };
            Ok(HttpResponse::Ok().json(resp))
        }
        Err(e) => Ok(e.error_response()),
    })
}

pub fn require_login(
    acc: permission::LoggedAccount,
) -> impl Future<Item = HttpResponse, Error = Error> {
    ok(HttpResponse::Ok().body(acc.0.to_string()))
}
