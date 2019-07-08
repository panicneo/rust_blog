use crate::errors::ServiceError;
use crate::schema::accounts;
use actix::Message;
use chrono::{Duration, Local, NaiveDateTime};
use jsonwebtoken::{Header, Validation};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Queryable, Debug, AsChangeset, PartialEq, Identifiable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub nickname: String,
    pub avatar: String,
    pub intro: String,
    pub permission: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

// 账户信息（客户端展示用）
#[derive(Debug, Clone, PartialEq, Queryable, Serialize, Deserialize)]
pub struct AccountItem {
    pub id: i64,
    pub nickname: String,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub permission: i16,
}

impl From<Account> for AccountItem {
    fn from(account: Account) -> Self {
        AccountItem {
            id: account.id,
            nickname: account.nickname,
            avatar: account.avatar,
            email: account.email,
            intro: account.intro,
            permission: account.permission,
        }
    }
}

// 用户注册
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSignUp {
    pub email: String,
    pub password: String,
}

impl Message for AccountSignUp {
    type Result = Result<AccountItem, ServiceError>;
}

// 用户登录
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSignIn {
    pub email: String,
    pub password: String,
}

impl Message for AccountSignIn {
    type Result = Result<AccountItem, ServiceError>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountSignInResp {
    pub token: String,
    pub account: AccountItem,
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

fn get_secret() -> String {
    dotenv::var("SECRET_KEY").unwrap_or_else(|_| "pkRiPd54NYEeHE)fjd".into())
}

pub fn encode_token(data: &AccountItem) -> Result<String, ServiceError> {
    let claims = Claims::new(data.id);
    jsonwebtoken::encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode".into()))
}

pub fn decode_token(token: &str) -> Result<i64, ServiceError> {
    jsonwebtoken::decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.uid))
        .map_err(|_err| ServiceError::Unauthorized)?
}
