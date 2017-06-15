use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let first = args.get(1);

    match first {
        Some(x) if x == "on" => println!("ON"),
        Some(x) if x == "off" => println!("OFF"),
        Some(x) => println!("Argument {} not recognized", x),
        None => writeln!(&mut stderr, "No argument")
                .expect("Could not write to stderr")
    };
}
