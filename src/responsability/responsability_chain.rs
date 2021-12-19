use crate::{Query, Response};
use std::error::Error;
use std::iter::Iterator;
use std::fmt::Debug;

pub trait Responsability: Debug {
    fn take(
        &self,
        iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query
    ) -> Result<Response, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct ResponsabilityChain {
    responsabilities: Vec<Box<dyn Responsability>>
}

impl ResponsabilityChain {
    pub fn new(responsabilities: Vec<Box<dyn Responsability>>) -> Self {
        ResponsabilityChain {
            responsabilities
        }
    }

    pub fn launch(&self, query: Query) -> Result<Response, Box<dyn Error>> {
        let mut iterator = self.responsabilities.iter();
        let responsability = iterator.next().unwrap();
        responsability.take(iterator, query)
    }
}
    
#[cfg(test)]
pub mod tests {
    use super::*;

    // ↓ This is a dumb Responsability used for testing
    #[derive(Debug)]
    pub struct CountChars {}

    impl Responsability for CountChars {
    fn take(
        &self,
        _iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query
    ) -> Result<Response, Box<dyn Error>> {
        Ok(Response {
            lines: vec![format!("{}", query.query.len())],
            query: query,
            })
        }
    }


    #[test]
    pub fn test_simple() {
        let chain = ResponsabilityChain::new (
            vec![
                Box::new(CountChars {}),
            ]
        );
        let query = Query { query: String::from("pikachu"), add_history: false, is_buffered: false };
        let response = chain.launch(query).unwrap();
        assert_eq!("7", &response.lines[0]);
    }
}
