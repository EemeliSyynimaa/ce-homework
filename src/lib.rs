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

const REG0: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const REG1: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data: Vec<u8> = fs::read(config.path.as_str()).unwrap();
    println!("; {}:", config.path);

    let command: u8 = data[0] >> 2;

    println!("bits 16");

    if command == MOV {
        let m: u8 = (data[1] & (128 + 64)) >> 6;

        if m == 3 {
            let d: u8 = data[0] & 2;
            let w: u8 = data[0] & 1;
            let reg: u8 = (data[1] & (32 + 16 + 8)) >> 3;
            let rm: u8 = data[1] & (4 + 2 + 1);

            let src: u8 = if d == 0 { reg } else { rm };
            let dst: u8 = if d == 0 { rm } else { reg };

            if w == 1 {
                println!("mov {}, {}", REG1[usize::from(dst)], REG1[usize::from(src)]);
            } else {
                println!("mov {}, {}", REG0[usize::from(dst)], REG0[usize::from(src)]);
            }
        }
    }

    Ok(())
}
