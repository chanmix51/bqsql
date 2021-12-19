use crate::bqclient::BqClient;
use crate::{Query, Response};
use rand::distributions::Alphanumeric;
use rand::Rng; 
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use super::*;

#[derive(Debug)]
pub struct BigQueryResponsability {
    bq_client: Box<dyn BqClient>,
    filename: String,
}

impl BigQueryResponsability {
    pub fn new(client: Box<dyn BqClient>) -> Self {
        let filename = {
            let tmpname = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
            ;
            format!("{}/bqsql_{}.sql", env::temp_dir().to_str().unwrap(), tmpname)
        };

        Self { bq_client: client, filename }
    }
}

impl Responsability for BigQueryResponsability {
    fn take(
        &self,
        _iterator: std::slice::Iter<Box<dyn Responsability>>,
        query: Query
    ) -> Result<Response, Box<dyn Error>> {
        let mut file = File::create(&self.filename)?;
        write!(file, "{}", &query.query)?;
        let lines = vec![self.bq_client.query(&self.filename)?];

        Ok(Response { query: query, lines: lines })
    }
}

impl Drop for BigQueryResponsability {
    fn drop(&mut self) {
        let filepath = PathBuf::from(&self.filename);

        if filepath.exists() {
            let _ = std::fs::remove_file(filepath);
        }
    }
}