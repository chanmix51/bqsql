pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod error;
mod query;
mod responsability;
mod response;
mod bqclient;

pub use response::Response;
pub use query::Query;
pub use responsability::*;
pub use bqclient::*;
pub use error::BqSqlError;
