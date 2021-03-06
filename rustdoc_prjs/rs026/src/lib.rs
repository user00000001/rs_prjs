use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Exception when generating config.")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
            println!("Now we query `{}` at {}\n", query, filename);
            Ok(Config {
                query,
                filename,
                case_insensitive,
            })
        }
        
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let result = fs::read_to_string(config.filename);
    match result {
        Ok(contents) => {
            // println!("{}", content);
            if config.case_insensitive {
                for line in search(&config.query, &contents) {
                    println!("{}", line);
                }
            } else {
                for line in search_case_insensitive(&config.query, &contents) {
                    println!("{}", line);
                }
            }
            
            Ok(())
        }
        Err(err) => Err(Box::new(err))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        )
    }
}