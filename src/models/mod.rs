pub mod account;
pub mod msg;
pub mod post;

use actix_web::Error;

pub trait Validate {
    fn validate(&self) -> Result<(), Error>;
}
