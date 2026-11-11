// grep -> Globally search a Regular Expression and Print
use minigrep::{Config, search, search_case_insensitive};
use std::{env, process};

fn main() {
    // collect method often needs annotation
    // taking command line args
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // dbg!(args);
    // reading contents

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
