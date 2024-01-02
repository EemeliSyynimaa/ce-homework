use std::env;
use std::fs;

const MOV: u8 = 0b100010;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    if args.len() == 2 {
        let path = args[1].as_str();
        let data: Vec<u8> = fs::read(path).unwrap();
        println!("Contents of file \"{}\":\n{:?}", path, data);

        let command: u8 = data[0] >> 2;

        if command == MOV {
            println!(
                "Command is MOV: {:b} vs {:b} (from {:b}",
                command, MOV, data[0]
            );
        }
    }
}
