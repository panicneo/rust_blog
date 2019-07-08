use crate::errors::ServiceError;
use crate::schema::accounts;
use actix::Message;
use chrono::NaiveDateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Queryable, Debug, AsChangeset, PartialEq, Identifiable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i64,
    pub password: String,
    pub nickname: String,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub permission: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
pub struct AccountReg {
    pub email: String,
    pub password: String,
}

impl Message for AccountReg {
    type Result = Result<AccountItem, ServiceError>;
}
