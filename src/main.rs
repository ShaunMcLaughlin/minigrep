// use to capture CLI commands
use std::env;
// use to read file
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};
// module to convert command line call into config struct
use minigrep::config::Config; 

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
     
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{

    for file in config.file_path {

        if config.include_file {
            println!("");
            println!("{}:", file);
            println!("---------------------------------");
        }
        
        let contents = fs::read_to_string(file)?;
        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }

    }

    Ok(())
}

