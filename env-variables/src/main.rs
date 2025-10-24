use std::env;
use std::error::Error;
use std::fs;
use std::process;

use env_variables::{search_case_insensitive,search};

fn main(){
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("existed in executing run func {e}");
        process::exit(1);
    }
}   

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build( args: &[String] ) -> Result<Config, & 'static str>{
        if args.len() < 3 {
            return Err("Need atleast 3 args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query: query, file_path: file_path, ignore_case: ignore_case })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    for line in result {
        println!("{}",line);
    }
    Ok(())
}
