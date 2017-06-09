use std::env;

fn main() {
    // Call .collect() to exhaust the iterator
    // Note that collect() can create a number of collections...
    // ...so it's good to annotate with Vec<String>.
    let args: Vec<String> = env::args().collect();
    // :? is the debug formatter
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
