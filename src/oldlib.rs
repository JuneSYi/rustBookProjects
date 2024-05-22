// use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// tied to oldmain.rs
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Box<dyn Error> is a trait object
// the function will return a type that implements the 'Error' trait
// 'dyn' keyword is short for dynamic, when we don't want to specify the particular type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // replaced the .expect() with ? operator
    // rather than 'panic!' on an error, '?' will return the error value from the current function for the caller
    // to handle
    // from std::fs library, read_to_string() takes an argument, opens the file, and returns a
    // std::io::Result<String> of the file's contents
    for line in results {
        println!("{line}");
    }

    // println!("With text:\n{contents}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // str slices used to refernce slices of the argument contents
    // meaning the output/expression will live as long as the data passed
    // into the function
    let mut results = Vec::new();
    for line in contents.lines() {
        // lines() returns an iterator
        if line.contains(query) {
            results.push(line);
        }
    }
    // vec![]
    results
}

// to_lowercase(), lowercases the query string and assign it to query
// calling to_lowercase() creates new data rather than referencing existing data, it's now a String rather than str
// because its now a String, when we pass it into the for loop below, we add a '&' because 'contains()' only takes string slices
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

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
