use std::env;
use std::error::Error;
use std::fs;
use std::process;

use mingrep::search;

fn main() {
    let args : Vec<String> = env::args().collect();
    // below is one by creating a plain method and calling it
    // let data = parse_data(&args);
    // calling a methods associated with a struct
    let data = Data::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    println!("Searching for {}",data.title);
    println!("In file {}",data.file_path);
    if let Err(e) = run(data) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

// fn run(input : Data) -> Result<(), Box<dyn Error>> {
//     let content = fs::read_to_string(input.file_path)?;
//     println!("content: {}", content);
//     Ok(())
// }

fn run(config: Data) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.title, &contents) {
        println!("{line}");
    }

    Ok(())
}

struct Data {
    title: String,
    file_path: String,
}

impl Data {
    fn build(args: &[String]) -> Result<Data, &'static str> {
        if args.len() < 3 {
            return Err("requires two args from user")
        }
        let title = args[1].clone();
        let path = args[2].clone();
        return Ok(Data { file_path: path, title })
    }
}

// fn parse_data(args: &[String]) -> Data {
//     let title = args[1].clone();
//     let file_path = args[2].clone();
//     return Data {title, file_path} 
// }