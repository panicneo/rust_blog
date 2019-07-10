use crate::database::DbAddr;
use crate::models::account::{AccountSignIn, AccountSignInResp, AccountSignUp};
use crate::utils::crypto::hashid;
use crate::utils::crypto::jwt;
use crate::utils::errors::ServiceError;
use actix_identity::Identity;
use actix_web::dev::Payload;
use actix_web::error::ResponseError;
use actix_web::web::{Data, Json};
use actix_web::{Error, FromRequest, HttpRequest, HttpResponse};
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

#[derive(Debug)]
pub struct LoggedAccount(i64);

impl FromRequest for LoggedAccount {
    type Error = Error;
    type Future = Result<LoggedAccount, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let uid = jwt::decode(&identity)?;
            let uid = hashid::decode(&uid)?;
            return Ok(LoggedAccount(uid));
        }
        Err(ServiceError::Unauthorized.into())
    }
}
