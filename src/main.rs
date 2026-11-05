// grep -> Globally search a Regular Expression and Print
use std::env;

fn main() {
    // collect method often needs annotation
    // taking command line args
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    // dbg!(args);
}
