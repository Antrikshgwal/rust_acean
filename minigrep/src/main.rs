use std::env;
use std::error::Error;
use std::fs;
use minigrep::Config;
use std::process;
fn main() {
    let args : Vec<String> = std::env::args().collect();
    dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }

    if let Err(e) = minigrep::run(config) {

    }
}
