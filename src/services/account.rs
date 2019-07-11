use crate::database::DbPool;
use crate::models::account::{Account, AccountItem, AccountSignIn, AccountSignUp};
use crate::utils::crypto::argon2;
use crate::utils::errors::ServiceError;
use actix::Handler;
use diesel::prelude::*;

impl Handler<AccountSignUp> for DbPool {
    type Result = Result<AccountItem, ServiceError>;

    fn handle(&mut self, msg: AccountSignUp, _: &mut Self::Context) -> Self::Result {
        use crate::schema::accounts::dsl::*;

        let conn = &self.0.get()?;
        let hashed = argon2::hash_password(&msg.password)?;
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
        if argon2::verify_password(&account.password, &msg.password)? {
            Ok(AccountItem::from(account))
        } else {
            Err(ServiceError::BadRequest("auth error".to_string()))
        }
    }
}
