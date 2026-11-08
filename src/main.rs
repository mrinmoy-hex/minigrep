// grep -> Globally search a Regular Expression and Print
use std::{env, fs, process};

fn main() {
    // collect method often needs annotation
    // taking command line args
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // dbg!(args);
    // reading contents
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // error message for args
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // using .clone() method just to deal with ownerships stuff
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
