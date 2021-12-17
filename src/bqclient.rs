use std::error::Error;
use std::process::Command;

pub struct BqClient;

impl BqClient {
    pub fn query(project_id: &str, sql_filename: &str) -> Result<String, Box<dyn Error>> {
        // launch bq with that file
        let output = Command::new("/usr/bin/bq")
            .arg("--project_id")
            .arg(project_id)
            .arg("query")
            .arg("--quiet")
            .arg("--use_legacy_sql=false")
            .arg("--flagfile")
            .arg(sql_filename)
            .output().unwrap();
        Ok(std::str::from_utf8(&output.stdout)?.to_string())
    }
}

