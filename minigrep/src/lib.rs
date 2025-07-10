use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

let matches = search(&config.query, &content); // matches is a vector string

if matches.is_empty(){
    println!("String doesnt found ");
}else{
    for line in matches {
        println!("{}",line);
    }
}


    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Error handling
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for lines in content.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }
    results
}

// Test driven devlopment

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe,fast,productive.
Pick three.
";

        assert_eq!(vec!["safe,fast,productive."], search(query, content));
    }
}
