use crate::database::DbPool;
use crate::errors::ServiceError;
use crate::models::account::{Account, AccountItem, AccountReg};
use actix::Handler;
use bcrypt::DEFAULT_COST;

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    let cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    bcrypt::hash(plain, cost)
        .map_err(|_| ServiceError::InternalServerError("hash error".to_string()))
}

impl Handler<AccountReg> for DbPool {
    type Result = Result<AccountItem, ServiceError>;

    fn handle(&mut self, msg: AccountReg, _: &mut Self::Context) -> Self::Result {
        use crate::schema::accounts::dsl::*;
        use diesel::prelude::*;

        let conn = &self.0.get()?;
        let hashed = hash_password(&msg.password)?;
        let account: Account = diesel::insert_into(accounts)
            .values((email.eq(&msg.email), password.eq(hashed)))
            .get_result(conn)?;
        Ok(AccountItem::from(account))
    }
}
