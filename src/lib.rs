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
        let disp = data[2] as i8;

        if d == 1 {
            println!("mov {}, [{} + {}]", reu, rmu, disp);
        } else {
            println!("mov [{} + {}], {}", rmu, disp, reu);
        }

        return 3;
    }

    // Memory mode, 16-bit displacement
    else if m == 2 {
        let disp = i16::from_le_bytes([data[2], data[3]]);

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
        let disp = data[1] as i8;
        println!("mov {}, {}", REG0[usize::from(reg)], disp);

        return 2;
    }

    // 16-bit immediate to register
    else {
        let disp = i16::from_le_bytes([data[1], data[2]]);
        println!("mov {}, {}", REG1[usize::from(reg)], disp);

        return 3;
    }
}

fn add_sub_cmp_register_memory_to_from_register(data: &[u8]) -> usize {
    let m: u8 = (data[1] & (128 + 64)) >> 6;
    let d: u8 = (data[0] & 2) >> 1;
    let w: u8 = data[0] & 1;
    let reg: u8 = (data[1] & (32 + 16 + 8)) >> 3;
    let rm: u8 = data[1] & (4 + 2 + 1);
    let reu = if w == 1 { REG1[usize::from(reg)] } else { REG0[usize::from(reg)] };
    let rmu = MOD0[usize::from(rm)];
    let sub_code = (data[0] & (32 + 16 + 8)) >> 3;
    let sub_command = match sub_code {
        0 => "add",
        5 => "sub",
        7 => "cmp",
        err => panic!("Incorrect sub command: {}!", err)
    };

    // Memory mode, no displacement
    if m == 0 {
        if d == 1 {
            println!("{} {}, [{}]", sub_command, reu, rmu);
        } else {
            println!("{} [{}], {}", sub_command, rmu, reu);
        }

        return 2;
    }

    // Memory mode, 8-bit displacement
    else if m == 1 {
        let disp = data[2] as i8;

        if d == 1 {
            println!("{} {}, [{} + {}]", sub_command, reu, rmu, disp);
        } else {
            println!("{} [{} + {}], {}", sub_command, rmu, disp, reu);
        }

        return 3;
    }

    // Memory mode, 16-bit displacement
    else if m == 2 {
        let disp = i16::from_le_bytes([data[2], data[3]]);

        if d == 1 {
            println!("{} {}, [{} + {}]", sub_command, reu, rmu, disp);
        } else {
            println!("{} [{} + {}], {}", sub_command, rmu, disp, reu);
        }

        return 4;
    }

    // Register mode, no displacement
    else if m == 3 {
        let src: u8 = if d == 0 { reg } else { rm };
        let dst: u8 = if d == 0 { rm } else { reg };

        if w == 1 {
            println!("{} {}, {}", sub_command, REG1[usize::from(dst)], REG1[usize::from(src)]);
        } else {
            println!("{} {}, {}", sub_command, REG0[usize::from(dst)], REG0[usize::from(src)]);
        }

        return 2;
    }

    panic!("add_sub_cmp_register_memory_to_from_register");
}

fn add_sub_cmp_immediate_to_register_memory(data: &[u8]) -> usize {
    let m: u8 = (data[1] & (128 + 64)) >> 6;
    let s: u8 = (data[0] & 2) >> 1;
    let w: u8 = data[0] & 1;
    let reg: u8 = (data[1] & (32 + 16 + 8)) >> 3;
    let rm: u8 = data[1] & (4 + 2 + 1);
    let _reu = if w == 1 { REG1[usize::from(reg)] } else { REG0[usize::from(reg)] };
    let rmu = MOD0[usize::from(rm)];
    let sub_code = (data[1] & (32 + 16 + 8)) >> 3;
    let sub_command = match sub_code {
        0 => "add",
        5 => "sub",
        7 => "cmp",
        err => panic!("Incorrect sub command: {}!", err)
    };

    let extend_sign = s == 0 && w == 1;

    // Memory mode, no displacement
    if m == 0 {
        // Direct address :-)
        let da = rm == 6;

        if da {
            let val = match extend_sign {
                true => i16::from_le_bytes([data[4], data[5]]),
                false => data[4] as i8 as i16
            };

            println!("{} [{}], {}", sub_command, i16::from_le_bytes([data[2], data[3]]), val);
        } else {
            let val = match extend_sign {
                true => i16::from_le_bytes([data[2], data[3]]),
                false => data[2] as i8 as i16
            };

            println!("{} [{}], {}", sub_command, rmu, val);
        }

        return 3 + extend_sign as usize * 1 + da as usize * 2;
    }

    // Memory mode, 8-bit displacement
    else if m == 1 {
        let disp = data[2] as i8;
        let val = match extend_sign {
            true => i16::from_le_bytes([data[3], data[4]]),
            false => data[3] as i8 as i16
        };


        println!("{} [{} + {}], {}", sub_command, rmu, disp, val);

        return 4 + extend_sign as usize * 1;
    }

    // Memory mode, 16-bit displacement
    else if m == 2 {
        let disp = i16::from_le_bytes([data[2], data[3]]);
        let val = match extend_sign {
            true => i16::from_le_bytes([data[4], data[5]]),
            false => data[4] as i8 as i16
        };

        println!("{} [{} + {}], {}", sub_command, rmu, disp, val);

        return 5 + extend_sign as usize * 1;
    }

    // Register mode, no displacement
    else if m == 3 {
        let val = match extend_sign {
            true => i16::from_le_bytes([data[2], data[3]]),
            false => data[2] as i8 as i16
        };

        println!("{} {}, {}", sub_command, REG1[usize::from(rm)], val);

        return 3 + extend_sign as usize * 1;
    }

    panic!("add_sub_cmp_immediate_to_register_memory");
}

fn add_sub_cmp_immediate_to_accumulator(data: &[u8]) -> usize {
    let w = data[0] & 1 == 1;
    let sub_code = (data[0] & (32 + 16 + 8)) >> 3;
    let sub_command = match sub_code {
        0 => "add",
        5 => "sub",
        7 => "cmp",
        err => panic!("Incorrect sub command: {}!", err)
    };

    let addr = match w {
        true => "ax",
        false => "al"
    };

    let val = match w {
        true => i16::from_le_bytes([data[1], data[2]]),
        false => data[1] as i8 as i16
    };

    println!("{} {}, {}", sub_command, addr, val);

    return 2 + w as usize * 1;
}

fn jump_equal_zero(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("je/jz, {}", val);
    return 2;
}

fn jump_less_not_greater_or_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jl/jnge, {}", val);
    return 2;
}

fn jump_less_or_equal_not_greater(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jle/jng, {}", val);
    return 2;
}

fn jump_below_equal_not_above_or_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jb/jnae, {}", val);
    return 2;
}

fn jump_below_or_equal_not_above(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jbe/jna, {}", val);
    return 2;
}

fn jump_parity_parity_even(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jp/jpe, {}", val);
    return 2;
}

fn jump_overflow(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jo, {}", val);
    return 2;
}

fn jump_sign(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("js, {}", val);
    return 2;
}

fn jump_not_equal_not_zero(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jne/jnz, {}", val);
    return 2;
}

fn jump_not_less_greater_or_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jnl/jge, {}", val);
    return 2;
}

fn jump_not_less_or_equal_greater(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jnle/jg, {}", val);
    return 2;
}

fn jump_not_below_above_or_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jnb/jae, {}", val);
    return 2;
}

fn jump_not_below_or_equal_above(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jnbe/ja, {}", val);
    return 2;
}

fn jump_not_par_par_odd(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jnp/jpo, {}", val);
    return 2;
}

fn jump_not_overflow(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jno, {}", val);
    return 2;
}

fn jump_not_sign(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jns, {}", val);
    return 2;
}

fn loop_cx_times(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("loop, {}", val);
    return 2;
}

fn loop_while_zero_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("loopz/loope, {}", val);
    return 2;
}

fn loop_while_not_zero_equal(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("loopnz/loopne, {}", val);
    return 2;
}

fn jump_cx_zero(data: &[u8]) -> usize {
    let val = data[1] as i8;
    println!("jcxz, {}", val);
    return 2;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let buf: Vec<u8> = fs::read(config.path.as_str()).unwrap();
    println!("; {}:", config.path);
    println!("bits 16 {:b}\n", 4834);

    let mut bytes_read: usize = 0;
    while bytes_read < buf.len() {
        let opcode: u8 = buf[bytes_read];
        let data = &buf[bytes_read..buf.len()];

        // MOV opcodes
        // Register/memory to/from register
        if opcode >> 2 == 0b100010 {
            bytes_read += mov_register_memory_to_from_register(data);
        }
        // Immediate to register
        // Note: we only care about the last four bits
        else if (opcode >> 4) == 0b1011 {
            bytes_read += mov_immediate_to_register(data)
        }

        // Add/sub/cmp opcodes
        // Register/memory to/from register
        else if opcode >> 2 == 0b000000 || opcode >> 2 == 0b001010 || opcode >> 2 == 0b001110 {
            bytes_read += add_sub_cmp_register_memory_to_from_register(data);
        }
        // Immediate to register/memory
        else if opcode >> 2 == 0b100000 {
            bytes_read += add_sub_cmp_immediate_to_register_memory(data);
        }
        // Immediate to accumulator
        else if opcode >> 2 == 0b000001 || opcode >> 2 == 0b001011 || opcode >> 2 == 0b001111 {
            bytes_read += add_sub_cmp_immediate_to_accumulator(data);
        }
        else if opcode == 0b01110100 {
            bytes_read += jump_equal_zero(data);
        }
        else if opcode == 0b01111100 {
            bytes_read += jump_less_not_greater_or_equal(data);
        }
        else if opcode == 0b01111110 {
            bytes_read += jump_less_or_equal_not_greater(data);
        }
        else if opcode == 0b01110010 {
            bytes_read += jump_below_equal_not_above_or_equal(data);
        }
        else if opcode == 0b01110110 {
            bytes_read += jump_below_or_equal_not_above(data);
        }
        else if opcode == 0b01111010 {
            bytes_read += jump_parity_parity_even(data);
        }
        else if opcode == 0b01110000 {
            bytes_read += jump_overflow(data);
        }
        else if opcode == 0b01111000 {
            bytes_read += jump_sign(data);
        }
        else if opcode == 0b01110101 {
            bytes_read += jump_not_equal_not_zero(data);
        }
        else if opcode == 0b01111101 {
            bytes_read += jump_not_less_greater_or_equal(data);
        }
        else if opcode == 0b01111111 {
            bytes_read += jump_not_less_or_equal_greater(data);
        }
        else if opcode == 0b01110011 {
            bytes_read += jump_not_below_above_or_equal(data);
        }
        else if opcode == 0b01110111 {
            bytes_read += jump_not_below_or_equal_above(data);
        }
        else if opcode == 0b01111011 {
            bytes_read += jump_not_par_par_odd(data);
        }
        else if opcode == 0b01110001 {
            bytes_read += jump_not_overflow(data);
        }
        else if opcode == 0b01111001 {
            bytes_read += jump_not_sign(data);
        }
        else if opcode == 0b11100010 {
            bytes_read += loop_cx_times(data);
        }
        else if opcode == 0b11100001 {
            bytes_read += loop_while_zero_equal(data);
        }
        else if opcode == 0b11100000 {
            bytes_read += loop_while_not_zero_equal(data);
        }
        else if opcode == 0b11100011 {
            bytes_read += jump_cx_zero(data);
        }
        else {
            panic!("Noo: {:b}", opcode);
        }
    }

    Ok(())
}
