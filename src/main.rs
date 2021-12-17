use bqsql::*;
use rand::distributions::Alphanumeric;
use rand::Rng; 
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{error::Error, path::PathBuf};
use std::env;
use std::fs::File;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct ApplicationParameters {
    #[structopt(long, short = "p", about = "GCP project identifier")]
    project_id: String,
    #[structopt(long, short = "d", about = "BigQuery dataset name")]
    dataset_id: String,
    #[structopt(long, parse(from_os_str), short = "j", about = "filepath of the JSON credential file", env="GOOGLE_CREDENTIALS")]
    credential_filepath: Option<PathBuf>,
}

struct Application {
    project_id: String,
    tmp_filename: String,
}

impl Application {
    pub fn new(parameters: ApplicationParameters) -> Application {
        let filename = {
            let tmpname = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect::<String>()
                ;
            format!("{}/bqsql_{}", env::temp_dir().to_str().unwrap(), tmpname)
        };

        Application {
            project_id: parameters.project_id,
            tmp_filename: filename,
        }
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    {
                        let mut file = File::create(&self.tmp_filename)?;
                        write!(file, "{}", line.as_str())?;
                    }
                    let output = BqClient::query(&self.project_id, &self.tmp_filename)?;
                    println!("{}", output);
                },
                Err(ReadlineError::Interrupted) => {
                    rl.save_history("history.txt").unwrap();

                    return Err(Box::new(ReadlineError::Interrupted))
                },
                Err(ReadlineError::Eof) => {
                    break
                },
                Err(err) => {
                    rl.save_history("history.txt").unwrap();

                    return Err(Box::new(err));
                }
            }
        }
        rl.save_history("history.txt").unwrap();
        std::fs::remove_file(&self.tmp_filename)?;

        Ok(())
    }
}

fn main() {
    let params = ApplicationParameters::from_args();
    Application::new(params)
        .execute()
        .unwrap_or_else(|err| { eprintln!("ERROR: {}", err); std::process::exit(1) });
}
