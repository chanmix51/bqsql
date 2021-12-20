use crate::*;

#[derive(Debug)]
pub struct BigClient {}

impl Responsability for BigClient {
    fn take(
        &self,
        mut iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query
    ) -> Result<Response> {
        println!("BigClient : Query = '{:?}'.", query);
        let query = if query.query.get(0..1).unwrap() == "\\" {
            match query.query.as_str() {
                "\\l" => {
                    let mut query = query.clone(); 
                    query.query = String::from("select schema_name from INFORMATION_SCHEMATA");
                    query
                },
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