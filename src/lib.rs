use std::{env, error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_string, &contents)
    } else {
        search(&config.search_string, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub struct Config {
    pub search_string: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let search_string = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // returns a tuple containing the search string and the file path
        Ok(Config {
            search_string,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.contains(query) {
            results.push(trimmed_line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // shadow the variable query
    let mut results: Vec<&str> = Vec::new();

    println!("query: {} {}", query, content);

    for line in content.lines() {
        let trimmed_line = line.trim();
        if trimmed_line.to_lowercase().contains(&query) {
            results.push(trimmed_line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duck tape.";

        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
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
