use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.search_string);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };

    // match run(config) {
    //     Ok(_) => (),
    //     Err(e) => {
    //         println!("Application error: {e}");
    //         process::exit(1);
    //     }
    // }
}
