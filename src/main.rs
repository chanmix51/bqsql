use bqsql::*;
use std::{error::Error, path::PathBuf};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Application {
    #[structopt(long, short = "p", about = "GCP project identifier")]
    project_id: String,
    #[structopt(long, short = "d", about = "BigQuery dataset name")]
    dataset_id: String,
    #[structopt(long, parse(from_os_str), short = "j", about = "filepath of the JSON credential file", env="GOOGLE_CREDENTIALS")]
    credential_filepath: Option<PathBuf>,
}

impl Application {
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
                    let output = BqClient::new(&self.project_id, &self.dataset_id)
                        .query(&line)?;
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

        Ok(())
    }
}
fn main() {
    Application::from_args()
        .execute()
        .unwrap_or_else(|err| { eprintln!("ERROR: {}", err); std::process::exit(1) });
}
