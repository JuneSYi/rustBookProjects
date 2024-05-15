use std::env;
use std::process;

use simple_command_line_tool::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // std::env::args function returns an iterator of the command line arguments passed
    // we can call the collect() method on an iterator to turn it into a collection, like a vector, that contains all
    // the elements the iterator produces

    let config = Config::build(&args).unwrap_or_else(|err| {
        // unwrap_or_else() allows us to defin some custom, non-pacnic! error handling
        // if the 'Result' is an 'Ok' value, this method's behavior is simila rto 'unwrap'
        // it returns the inner value 'Ok' is wrapping
        // if the value is an 'Err' value, this method calls the code in the 'closure'

        println!("Problem parsing arguments: {err}");
        process::exit(1);
        // process::exit function will stop the program immediately and return the number that was passed
        // as the exit status code. simila rto panic!, but no longer all the extra output
    });

    // let (query, file_path) = parse_config(&args);
    // saves the first and 2nd arguments

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // run(config);
    if let Err(e) = simple_command_line_tool::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
