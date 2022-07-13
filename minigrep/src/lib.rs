use std::env;
use std::fs;
use std::process;
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if (args.len() < 3) {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("run config {:#?}", config);
    let contents = fs::read_to_string(config.file_name)?;
    println!("file content {}", contents);
    Ok(())
}
