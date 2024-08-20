use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(query: &str, file: &str) -> Config {
        let case_sensitive = env::var("CASE_SENSITIVE")
            .map(|val| val.to_lowercase() == "true") // Checking if the env variable is set to "true"
            .unwrap_or(false); // Default to false if not set
        Config {
            query: query.to_string(),
            file: file.to_string(),
            case_sensitive,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// Case-insensitive search function
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // Convert the query once
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
Pick three
Duct Tape.";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
