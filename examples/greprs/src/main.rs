extern crate greprs;

use std::env;
use std::io::prelude::*;
use std::process;

use greprs::Config;

fn main() {
    // Call .collect() to exhaust the iterator
    // Note that collect() can create a number of collections...
    // ...so it's good to annotate with Vec<String>.
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(&args).unwrap_or_else(|err| {
        writeln!(
            &mut stderr,
            "Problem parsing arguments: {}",
            err
        ).expect("Could not write to stderr");
        process::exit(1);
    });

    // Here, we can `if let ...` since unwrap_or_else doesn't make sense
    // as we don't care to unwrap () in the Ok case
    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "Application error: {}",
            e
        ).expect("Could not write to stderr");
        process::exit(1);
    }
}
