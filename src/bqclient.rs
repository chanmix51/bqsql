use rand::Rng; 
use rand::distributions::Alphanumeric;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct BqClient {
    project_id: String,
    dataset_id: String,
}

impl BqClient {
    pub fn new(project_id: &str, dataset_id: &str) -> BqClient {
        BqClient {
            project_id: String::from(project_id),
            dataset_id: String::from(dataset_id),
        }
    }

    pub fn query(&self, sql: &str) -> Result<String, Box<dyn Error>> {
        // save the query in a file
        let filename = {
            let tmpname = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect::<String>()
                ;
            let f = format!("{}/bqsql_{}", env::temp_dir().to_str().unwrap(), tmpname);
            let mut file = File::create(&f)?;
            write!(file, "{}", sql)?;
            f
        };
        // launch bq with that file
        let output = Command::new("/usr/bin/bq")
            .arg("--project_id")
            .arg(&self.project_id)
            .arg("query")
            .arg("--quiet")
            .arg("--use_legacy_sql=false")
            .arg("--flagfile")
            .arg(filename)
            .output().unwrap();
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }
}

