use std::env;
use std::fs;
use std::process;

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
    run(data);
}

fn run(input : Data){
    let data = fs::read_to_string(input.file_path).expect("Should read the file");
    println!("content: {}", data);
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