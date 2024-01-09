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

fn process_mov(command: u8, data: u8) {
    // println!("; {:b} {:b}", command, data);

    let m: u8 = (data & (128 + 64)) >> 6;

    if m == 3 {
        let d: u8 = command & 2;
        let w: u8 = command & 1;
        let reg: u8 = (data & (32 + 16 + 8)) >> 3;
        let rm: u8 = data & (4 + 2 + 1);

        let src: u8 = if d == 0 { reg } else { rm };
        let dst: u8 = if d == 0 { rm } else { reg };

        if w == 1 {
            println!("mov {}, {}", REG1[usize::from(dst)], REG1[usize::from(src)]);
        } else {
            println!("mov {}, {}", REG0[usize::from(dst)], REG0[usize::from(src)]);
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let data: Vec<u8> = fs::read(config.path.as_str()).unwrap();
    println!("; {}:", config.path);
    println!("bits 16\n");
    for i in 0..data.len() / 2 {
        let c_idx: usize = i * 2;
        let b_idx: usize = c_idx + 1;

        let command: u8 = data[c_idx] >> 2;

        if command == MOV {
            process_mov(data[c_idx], data[b_idx]);
        }
    }

    Ok(())
}
