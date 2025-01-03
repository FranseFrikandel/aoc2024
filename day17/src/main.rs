use std::time::Instant;
use std::fs::read_to_string;

enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Opcode::ADV,
            1 => Opcode::BXL,
            2 => Opcode::BST,
            3 => Opcode::JNZ,
            4 => Opcode::BXC,
            5 => Opcode::OUT,
            6 => Opcode::BDV,
            7 => Opcode::CDV,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn main() {
    let timer = Instant::now();
    let mut reg_a: usize = 0;
    let mut reg_b: usize = 0;
    let mut reg_c: usize = 0;
    let mut program_mem: Vec<usize> = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        if line.starts_with("Register A:") {reg_a = line[12..].parse().expect("Failed to read register A");}
        if line.starts_with("Register B:") {reg_b = line[12..].parse().expect("Failed to read register B");}
        if line.starts_with("Register C:") {reg_c = line[12..].parse().expect("Failed to read register C");}
        if line.starts_with("Program:") {
            let program_str = &line[9..];
            for opcode_str in program_str.split(",") {
                let opcode: usize = opcode_str.parse().expect("Failed to read program memory");
                program_mem.push(opcode);
            }
        }
    }

    let out = run_machine(&program_mem, reg_a, reg_b, reg_c, None);
    println!("Part 1: {:?}", out);

    let mut i = 0;
    loop {
        let outp_prog = run_machine(&program_mem, i, reg_b, reg_c, Some(&program_mem));
        if outp_prog == program_mem{
            println!("Part 2: {}", i);
            break;
        }
        i += 1;
    }

    println!("Total runtime: {:.3?}", timer.elapsed());
}

fn get_operand(operand: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    match operand {
        0..4 => operand as usize,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => panic!("Invalid operand")
    }
}

fn run_machine(prog_mem: &Vec<usize>, reg_a: usize, reg_b: usize, reg_c: usize, correct_output: Option<&Vec<usize>>) -> Vec<usize> {
    let mut reg_a = reg_a;
    let mut reg_b = reg_b;
    let mut reg_c = reg_c;
    let mut pc = 0;
    let mut out: Vec<usize> = Vec::new();

    while pc < prog_mem.len() {
        let opcode = Opcode::from(prog_mem[pc]);
        match opcode {
            Opcode::ADV => {
                reg_a = reg_a / 2_usize.pow(get_operand(prog_mem[pc+1], reg_a, reg_b, reg_c).try_into().unwrap());
            },
            Opcode::BXL => {
                reg_b = reg_b ^ prog_mem[pc+1];
            },
            Opcode::BST => {
                reg_b = get_operand(prog_mem[pc+1], reg_a, reg_b, reg_c) % 8;
            },
            Opcode::JNZ => {
                if reg_a != 0 {
                    pc = prog_mem[pc+1];
                    continue;
                }
            },
            Opcode::BXC => {
                reg_b = reg_b ^ reg_c
            },
            Opcode::OUT => {
                let operand = get_operand(prog_mem[pc+1], reg_a, reg_b, reg_c);
                let out_val = (operand % 8) as usize;
                out.push(out_val);
                if let Some(corr_output) = correct_output {
                    if corr_output[out.len() - 1] != out_val {
                        break;
                    }
                }
            },
            Opcode::BDV => {
                reg_b = reg_a / 2_usize.pow(get_operand(prog_mem[pc+1], reg_a, reg_b, reg_c).try_into().unwrap());
            },
            Opcode::CDV => {
                reg_c = reg_a / 2_usize.pow(get_operand(prog_mem[pc+1], reg_a, reg_b, reg_c).try_into().unwrap());
            }
        }
        pc += 2;
    }
    return out;
}