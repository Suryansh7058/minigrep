// use std::env;
// use std::process;

// use minigrep::Config;
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
//     println!("Searching for {} in {}", config.query, config.file);
//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application Error: {}", e);
//         process::exit(1);
//     }
// }

use clap::{Arg, Command};
use minigrep::Config;
use std::process;

fn main() {
    let matches = Command::new("minigrep")
        .version("1.0")
        .author("Suryansh Singh <suryanshsingh989@gmail.com>")
        .about("Searches for a query in a file and displays matching lines.")
        .arg(
            Arg::new("query")
                .help("The query string to search for")
                .required(true)
                .index(1), // First positional argument
        )
        .arg(
            Arg::new("file")
                .help("The file to search in")
                .required(true)
                .index(2), // Second positional argument
        )
        .get_matches();

    // Collect command-line arguments
    let query = matches.get_one::<String>("query").expect("query required");
    let file = matches.get_one::<String>("file").expect("file required");

    // Create the config using query and file
    let config = Config::new(query, file);

    // Print search details
    println!("Searching for '{}' in file '{}'", config.query, config.file);

    // Run the application and handle any errors
    if let Err(e) = minigrep::run(config) {
        eprintln!("\x1b[31mApplication error: {}\x1b[0m", e);
        process::exit(1);
    }
}
