use std::env;
use std::process;

use simple_command_line_tool::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // env::args function returns an iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = simple_command_line_tool::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
