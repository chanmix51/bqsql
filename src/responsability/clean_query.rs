use crate::{Query, Response, Responsability};
use std::error::Error;

#[derive(Debug)]
pub struct CleanQuery {}

impl Responsability for CleanQuery {
    fn take(
        &self,
        mut iterator: std::slice::Iter<Box<dyn Responsability>>,
        mut query: Query
    ) -> Result<Response, Box<dyn Error>> {
        let str_query = (&query.query).trim();
        query.query = String::from(str_query);

        if query.query.len() == 0 {
            query.add_history = false;

            Ok(
                Response {
                    lines: vec![],
                    query: query
                }
            )
        } else {
            let resp = iterator.next().unwrap();
        
            resp.take(iterator, query)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResponsabilityChain;
    use crate::responsability::responsability_chain::tests::CountChars;

    #[test]
    fn test_trim() {
        let chain = ResponsabilityChain::new(vec![
            Box::new(CleanQuery {}),
            Box::new(CountChars {}),
        ]);

        let response = chain.launch(Query::new("whatever", "whatever", "  ein  ")).unwrap();
        assert_eq!("3", response.lines[0]);
    }
}