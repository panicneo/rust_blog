use crate::utils::crypto::{hashid, jwt};
use crate::utils::errors::ServiceError;
use actix_web::dev::Payload;
use actix_web::http::header;
use actix_web::{Error, FromRequest, HttpRequest};

#[derive(Debug)]
pub struct LoggedAccount(pub i64);

impl FromRequest for LoggedAccount {
    type Error = Error;
    type Future = Result<LoggedAccount, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(token) = req.headers().get(header::AUTHORIZATION) {
            let token_str = token.to_str().map_err(|_| ServiceError::Unauthorized)?;
            let uid = jwt::decode(token_str)?;
            let uid = hashid::decode(&uid)?;
            return Ok(LoggedAccount(uid));
        }
        Err(ServiceError::Unauthorized.into())
    }
}
