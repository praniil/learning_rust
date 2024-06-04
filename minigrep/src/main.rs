use std::error::Error;
use std::{env, fs};
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config{
    //constructor functions name:new
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query: query, file_path: file_path })
    }
}
fn main() {
    println!("Hello, world!");
    //env::args() := is an iterator, that produces a series of values and we can call the collect method on an iterator to turn it into collection
    let args: Vec<String> = env::args().collect();
    //programs name takes the first value:= args[0]
    let configuration = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let configuration: Config = parse_config(&args);

    println!("Searching for: {})", configuration.query);
    println!("In file: {}", configuration.file_path);
    if let Err(e) = run(configuration) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(()) 
}
