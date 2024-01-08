use std::env;
use std::process;

use ce_homework::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = ce_homework::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
