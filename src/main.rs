use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    if args.len() == 2 {
        let path = args[1].as_str();
        let data: Vec<u8> = fs::read(path).unwrap();
        println!("Contents of file \"{}\":\n{:?}", path, data);
    }
}
