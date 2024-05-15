use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // std::env::args function returns an iterator of the command line arguments passed
    // we can call the collect() method on an iterator to turn it into a collection, like a vector, that contains all
    // the elements the iterator produces

    // dbg!(args);
    // debugging macro that prints the value of an expression along with the file name and line number where the macro is called
    // it returns the value of the expression

    let query = &args[1];
    // save the first argument
    let file_path = &args[2];
    // save the second argument

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // from std::fs library, read_to_string() takes an argument, opens the file, and returns a
    // std::io::Result<String> of the file's contents

    println!("With text:\n{contents}");
}
