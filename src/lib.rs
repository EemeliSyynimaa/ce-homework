use std::error::Error;
use std::{fs, string};

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

const REG0: [&str; 8] = ["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const REG1: [&str; 8] = ["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const MOD0: [&str; 8] = ["bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx"];

fn mov_register_memory_to_from_register(data: &[u8]) -> usize {
    // println!("mov register/memory to/from register ");
    // println!("; {:b} {:b} {:b} {:b}", data[0], data[1], data[2], data[3]);

    let m: u8 = (data[1] & (128 + 64)) >> 6;
    let d: u8 = (data[0] & 2) >> 1;
    let w: u8 = data[0] & 1;
    let reg: u8 = (data[1] & (32 + 16 + 8)) >> 3;
    let rm: u8 = data[1] & (4 + 2 + 1);
    let reu = if w == 1 { REG1[usize::from(reg)] } else { REG0[usize::from(reg)] };
    let rmu = MOD0[usize::from(rm)];

    // Memory mode, no displacement
    if m == 0 {

        if d == 1 {
            println!("mov {}, [{}]", reu, rmu);
        } else {
            println!("mov [{}], {}", rmu, reu);
        }

        return 2;
    }

    // Memory mode, 8-bit displacement
    else if m == 1 {
        let disp = data[2];

        if d == 1 {
            println!("mov {}, [{} + {}]", reu, rmu, disp);
        } else {
            println!("mov [{} + {}], {}", rmu, disp, reu);
        }

        return 3;
    }

    // Memory mode, 16-bit displacement
    else if m == 2 {
        let disp: u16 = u16::from_le_bytes([data[2], data[3]]);

        if d == 1 {
            println!("mov {}, [{} + {}]", reu, rmu, disp);
        } else {
            println!("mov [{} + {}], {}", rmu, disp, reu);
        }

        return 4;
    }

    // Register mode, no displacement
    else if m == 3 {
        let src: u8 = if d == 0 { reg } else { rm };
        let dst: u8 = if d == 0 { rm } else { reg };

        if w == 1 {
            println!("mov {}, {}", REG1[usize::from(dst)], REG1[usize::from(src)]);
        } else {
            println!("mov {}, {}", REG0[usize::from(dst)], REG0[usize::from(src)]);
        }

        return 2;
    }

    panic!("mov_register_memory_to_from_register");
}

fn mov_immediate_to_register(data: &[u8]) -> usize {
    // println!("mov immediate to register ");

    let w: u8 = data[0] & 8;
    let reg: u8 = data[0] & (1 + 2 + 4);

    // 8-bit immediate to register
    if w == 0 {
        let disp: u8 = data[1] as u8;
        println!("mov {}, {}", REG0[usize::from(reg)], disp);

        return 2;
    }

    // 16-bit immediate to register
    else {
        let disp: u16 = u16::from_le_bytes([data[1], data[2]]);
        println!("mov {}, {}", REG1[usize::from(reg)], disp);

        return 3;
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let buf: Vec<u8> = fs::read(config.path.as_str()).unwrap();
    println!("; {}:", config.path);
    println!("bits 16\n");

    let mut bytes_read: usize = 0;
    while bytes_read < buf.len() {
        let opcode: u8 = buf[bytes_read] >> 2;
        let data = &buf[bytes_read..buf.len()];

        // MOV opcodes
        // Register/memory to/from register
        if opcode == 0b100010 {
            bytes_read += mov_register_memory_to_from_register(data);
        }

        // Immediate to register
        // Note: we only care about the last four bits
        else if (opcode >> 2) == 0b1011 {
            bytes_read += mov_immediate_to_register(data)
        }
        else {
            panic!("Noo");
        }
    }

    Ok(())
}
