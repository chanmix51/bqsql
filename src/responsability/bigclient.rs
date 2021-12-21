use crate::*;

#[derive(Debug)]
pub struct BigClient {}

impl Responsability for BigClient {
    fn take(
        &self,
        mut iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query,
    ) -> Result<Response> {
        let query = if query.query.get(0..1).unwrap() == "\\" {
            match query.query.as_str() {
                "\\l" => {
                    let mut query = query.clone();
                    query.query = format!(
                        "select catalog_name as project_name, schema_name as dataset_name, location from `INFORMATION_SCHEMA.SCHEMATA`"
                    );
                    query.add_history = false;
                    query
                },
                "\\dt" => {
                    let mut query = query.clone();
                    query.query = format!(
                        "select table_catalog as project_id, table_schema, table_name as dataset_id from `{}.INFORMATION_SCHEMA.TABLES` where table_type = 'BASE TABLE'",
                        query.dataset_id
                    );
                    query.add_history = false;
                    query
                }
                msg => {
                    let error = format!("syntax error, unknown pattern '{}'.", msg);

                    return Err(BqSqlError::Standard(error).into());
                }
            }
        } else {
            query
        };
        let responsability = iterator.next().unwrap();

        responsability.take(iterator, query)
    }
}
