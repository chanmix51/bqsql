use std::ops::Deref;

#[derive(Debug)]
enum BqSqlError {
    BqError(Box<dyn std::error::Error>),
    Standard(String),
}
impl std::fmt::Display for BqSqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BqSqlError::BqError(e) => write!(f, "{}", e),
            BqSqlError::Standard(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for BqSqlError {}