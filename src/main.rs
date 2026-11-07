// grep -> Globally search a Regular Expression and Print
use std::{env, fs};

fn main() {
    // collect method often needs annotation
    // taking command line args
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // dbg!(args);
    // reading contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // using .clone() method just to deal with ownerships stuff
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
