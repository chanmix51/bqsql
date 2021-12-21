use bqsql::*;

#[derive(Debug)]
struct BigQueryResponsabilityMock {}

impl Responsability for BigQueryResponsabilityMock {
    fn take(
        &self,
        _iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query
    ) -> Result<Response> {
       Ok(Response { query, lines: vec![String::from("pika")]})
    }
}

#[test]
fn simple_application() {
    let chain = ResponsabilityChain::new(
        vec![
            Box::new(BigQueryResponsabilityMock {})
        ]
    );
    let query = Query::new("chatever", "whatever", "chu");
    let response = chain.launch(query).unwrap();

    assert_eq!(String::from("pika"), response.lines[0]);
}