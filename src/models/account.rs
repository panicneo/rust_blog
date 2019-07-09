use crate::schema::accounts;
use crate::utils::crypto::hashid_encode;
use crate::utils::errors::ServiceError;
use actix::Message;
use chrono::NaiveDateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Queryable, Debug, AsChangeset, Identifiable, Insertable)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountItem {
    pub id: String,
    pub nickname: String,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub permission: i16,
}

impl From<Account> for AccountItem {
    fn from(account: Account) -> Self {
        AccountItem {
            id: hashid_encode(account.id).unwrap(),
            nickname: account.nickname,
            avatar: account.avatar,
            email: account.email,
            intro: account.intro,
            permission: account.permission,
        }
    }
}

// 用户注册
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSignUp {
    pub email: String,
    pub password: String,
}

impl Message for AccountSignUp {
    type Result = Result<AccountItem, ServiceError>;
}

// 用户登录
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSignIn {
    pub email: String,
    pub password: String,
}

impl Message for AccountSignIn {
    type Result = Result<AccountItem, ServiceError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSignInResp {
    pub token: String,
    pub account: AccountItem,
}
