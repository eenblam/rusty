use std::env;
use std::fs::File;
// Traits for IO
use std::io::prelude::*;
use std::process;

fn main() {
    // Call .collect() to exhaust the iterator
    // Note that collect() can create a number of collections...
    // ...so it's good to annotate with Vec<String>.
    let args: Vec<String> = env::args().collect();
    // :? is the debug formatter
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename
        })
    }
}
