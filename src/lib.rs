use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box<dyn Error> is a trait object
    // the function will return a type that implements the 'Error' trait
    // 'dyn' keyword is short for dynamic, when we don't want to specify the particular type
    let contents =
        // fs::read_to_string(config.file_path).expect("Should have been able to read the file");
        fs::read_to_string(config.file_path)?;
    // replaced the .expect() with ? operator
    // rather than 'panic!' on an error, '?' will return the error value from the current function for the caller
    // to handle
    // from std::fs library, read_to_string() takes an argument, opens the file, and returns a
    // std::io::Result<String> of the file's contents
    println!("With text:\n{contents}");

    Ok(())
}
