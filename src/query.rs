#[derive(Debug)]
pub struct Query {
    pub query: String,
    pub add_history: bool,
    pub is_buffered: bool,
}

impl Query {
    pub fn new(query: &str) -> Query {
        Query {
            query: String::from(query),
            add_history: true,
            is_buffered: true,
        }
    }
}