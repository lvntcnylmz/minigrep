//! # minigrep
//!
//! A small library for searching case-sensitive and case-insensitive regexes
use colored::*;
use std::{env, error::Error, fs};

/// Holds information such as regex to search for, file to search for and case sensitivity.
pub struct Config {
    /// string to search for in file
    pub query: String,
    /// file path of the file to search
    pub file_path: String,
    /// case sensitivity option
    pub ignore_case: bool,
}

impl Config {
    /// # Errors
    ///
    /// This function returns an error if there are not enough arguments.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Self {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// # Errors
///
/// This function will return an error if path does not already exist.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line.green());
    }

    Ok(())
}

/// Case-sensitive search for a query in a specified file.
///
/// ## Usage
///
/// `search(query to search, file_path or file)`
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// Case-insensitive search for a query in a specified file.
///
/// ## Usage:
///
/// `search_case_insensitive(query to search, file_path or filez)`
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
        );
    }
}
