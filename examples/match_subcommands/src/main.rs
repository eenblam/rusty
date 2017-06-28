use std::env;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let input = Input::new(args).unwrap_or_else(|err| {
        writeln!(&mut stderr,
                 "Problem parsing arguments: {}",
                 err
        ).expect("Could not write to stderr");
        process::exit(1);
    });
    //println("{}", input.args);
}

enum Command {
    On,
    Off,
    List,
    Err,
}

struct Input {
    command: Command,
    args: Vec<String>,
}

impl Input {
    fn new(args: Vec<String>) -> Result<Input, &'static str> {
        let first = args.get(1);

        let command = match first {
            //TODO slice args[2:]
            Some(x) if x == "on" => Command::On,
            Some(x) if x == "off" => Command::Off,
            Some(x) => Command::Err("Argument {} not recognized", x),
            None => Command::Err("No argument"),
        };

        //let last_args = args.iter().drop(2).clone();
        //let last_args = args[2..].clone();
        let last_args = args.into_iter().skip(2).shrink_to_fit();

        match command {
            Command::Err(s) => Err(s),
            _ => Ok(Input { command: command, args: last_args, }),
        }
        
    }
}
