#[derive(Debug, Clone)]
pub struct Query {
    pub project_id: String,
    pub dataset_id: String,
    pub query: String,
    pub add_history: bool,
    pub is_buffered: bool,
}

impl Query {
    pub fn new(project_id: &str, dataset_id: &str, query: &str) -> Query {
        Query {
            project_id: String::from(project_id),
            dataset_id: String::from(dataset_id),
            query: String::from(query),
            add_history: true,
            is_buffered: true,
        }
    }
}
