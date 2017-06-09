extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    // Call .collect() to exhaust the iterator
    // Note that collect() can create a number of collections...
    // ...so it's good to annotate with Vec<String>.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Here, we can `if let ...` since unwrap_or_else doesn't make sense
    // as we don't care to unwrap () in the Ok case
    if let Err(e) = greprs::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
