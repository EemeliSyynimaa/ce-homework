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
                "Command is MOV: {:b} vs {:b} (from {:b}) with data {:b}",
                command, MOV, data[0], data[1]
            );

            let d: u8 = data[0] & 2;
            let w: u8 = data[0] & 1;
            let m: u8 = (data[1] & (128 + 64)) >> 6;
            let reg: u8 = (data[1] & (32 + 16 + 8)) >> 3;
            let rm: u8 = data[1] & (4 + 2 + 1);

            println!("{:b}{:b}{:b} {:b}{:03b}{:03b}", command, d, w, m, reg, rm);
        }
    }
}
