use bqsql::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct ApplicationParameters {
    #[structopt(long, short = "p", about = "GCP project identifier")]
    project_id: String,
    #[structopt(long, short = "d", about = "BigQuery dataset name")]
    dataset_id: String,
    #[structopt(long, parse(from_os_str), short = "j", about = "filepath of the JSON credential file", env="GOOGLE_APPLICATION_CREDENTIALS")]
    credential_filepath: PathBuf,
}

struct Application {
    chain: ResponsabilityChain
}

impl Application {
    pub fn new(params: ApplicationParameters) -> Self {
        let chain = ResponsabilityChain::new(vec![
            Box::new(CleanQueryResponsability {}),
            Box::new(BigQueryResponsability::new(Box::new(BqBinary::new(&params.project_id))))
        ]);

        Self { chain }
    }

    pub fn execute(&self) -> Result<()> {
        // `()` can be used when no completer is required
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    let query = Query::new(&line);
                    let response = self.chain.launch(query)?;

                    if response.query.add_history {
                        rl.add_history_entry(line.as_str());
                    }
                    for line in response.lines {
                        println!("{}", line);
                    }
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
    let params = ApplicationParameters::from_args();
    Application::new(params)
        .execute()
        .unwrap_or_else(|err| { eprintln!("ERROR: {}", err); std::process::exit(1) });

}
