use std::env;
use std::process;

use minigrep::Config;

fn main() {

    //? specifying the type here so collect knows to return a vector of Strings
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //? if let is used here because run does not return a value that we want to unwrap, as we do in the
    //? unwrap_or_else call on Config::new
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1)
    }
}
