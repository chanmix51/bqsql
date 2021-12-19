use super::Query;

#[derive(Debug)]
pub struct Response {
    pub lines: Vec<String>,
    pub query: Query,
}