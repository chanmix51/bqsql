use super::Query;

pub struct Response {
    pub lines: Vec<String>,
    pub query: Query,
}