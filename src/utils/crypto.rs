use crate::utils::config;
use crate::utils::errors::ServiceError;
use argonautica::{Hasher, Verifier};
use chrono::{Duration, Local};
use jsonwebtoken::{Header, Validation};
use serde_derive::{Deserialize, Serialize};

lazy_static::lazy_static! {
   pub static ref SECRET_KEY: String = config::get("SECRET_KEY", "SECRET_KEY".to_owned());
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    Hasher::default()
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .hash()
        .map_err(|err| {
            dbg!(err);
            ServiceError::InternalServerError("hash_password".to_string())
        })
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, ServiceError> {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(SECRET_KEY.as_str())
        .verify()
        .map_err(|err| {
            dbg!(err);
            ServiceError::Unauthorized
        })
}

// jwt Token auth: Claim, token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    // issuer
    pub sub: String,
    // subject
    pub iat: i64,
    // issued at
    pub exp: i64,
    // expiry
    pub uid: i64,
    // user id
}

// claims's constructor
impl Claims {
    pub fn new(uid: i64) -> Self {
        Claims {
            iss: "rust_blog".into(),
            sub: "auth".into(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24 * 5)).timestamp(),
            uid,
        }
    }
}

pub fn encode_token(id: i64) -> Result<String, ServiceError> {
    let claims = Claims::new(id);
    jsonwebtoken::encode(&Header::default(), &claims, SECRET_KEY.as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode".into()))
}

pub fn decode_token(token: &str) -> Result<i64, ServiceError> {
    jsonwebtoken::decode::<Claims>(token, SECRET_KEY.as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.uid))
        .map_err(|_err| ServiceError::Unauthorized)?
}

pub fn hashid_encode(id: i64) -> Result<String, ServiceError> {
    let harsher = harsh::HarshBuilder::new()
        .salt(SECRET_KEY.as_ref() as &str)
        .length(10)
        .init()?;
    harsher
        .encode(&[id as u64])
        .ok_or_else(|| ServiceError::InternalServerError("harsh encode".into()))
}

pub fn hashid_decode(hash_id: &str) -> Result<i64, ServiceError> {
    let harsher = harsh::HarshBuilder::new()
        .salt(SECRET_KEY.as_ref() as &str)
        .length(10)
        .init()?;
    harsher
        .decode(hash_id)
        .map(|vec| vec[0] as i64)
        .ok_or_else(|| ServiceError::InternalServerError("harsh decode".into()))
}
