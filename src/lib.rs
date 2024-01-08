use std::error::Error;
use std::fs;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough parameters");
        }

        let path = args[1].clone();

        Ok(Config { path })
    }
}

const MOV: u8 = 0b100010;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data: Vec<u8> = fs::read(config.path.as_str()).unwrap();
    println!("Contents of file \"{}\":\n{:?}", config.path, data);

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

    Ok(())
}
