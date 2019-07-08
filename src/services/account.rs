use crate::database::DbPool;
use crate::errors::ServiceError;
use crate::models::account::{Account, AccountItem, AccountSignIn, AccountSignUp};
use actix::Handler;
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    let cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(5),
        _ => 5,
    };
    bcrypt::hash(plain, cost)
        .map_err(|_| ServiceError::InternalServerError("hash error".to_string()))
}

impl Handler<AccountSignUp> for DbPool {
    type Result = Result<AccountItem, ServiceError>;

    fn handle(&mut self, msg: AccountSignUp, _: &mut Self::Context) -> Self::Result {
        use crate::schema::accounts::dsl::*;

        let conn = &self.0.get()?;
        let hashed = hash_password(&msg.password)?;
        let account: Account = diesel::insert_into(accounts)
            .values((email.eq(&msg.email), password.eq(hashed)))
            .get_result(conn)?;
        Ok(AccountItem::from(account))
    }
}

impl Handler<AccountSignIn> for DbPool {
    type Result = Result<AccountItem, ServiceError>;

    fn handle(&mut self, msg: AccountSignIn, _: &mut Self::Context) -> Self::Result {
        use crate::schema::accounts::dsl::*;
        let conn = &self.0.get()?;
        let account = accounts
            .filter(email.eq(&msg.email))
            .get_result::<Account>(conn)?;
        let hashed = hash_password(&msg.password)?;
        match bcrypt::verify(&msg.password, &hashed) {
            Ok(_) => Ok(AccountItem::from(account)),
            Err(_) => Err(ServiceError::BadRequest("auth error".to_string())),
        }
    }
}
