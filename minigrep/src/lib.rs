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
    // println!("run config {:#?}", config);
    let contents = fs::read_to_string(config.file_name)?;
    // println!("file content {}", contents);
    for line in search(&config.query, &contents){
        println!("matched {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // println!("line {}", line);
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct"; 
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}