mod ffi;

use crate::ffi as linenoise;
use ckb_vm::{
    instructions::execute_instruction, machine::VERSION1, DefaultCoreMachine,
    DefaultMachineBuilder, Machine, Memory, SparseMemory, WXorXMemory, ISA_B, ISA_IMC, ISA_V,
};
use ckb_vm_contrib::assembler::parse;
use std::str::FromStr;

fn main() {
    println!("Welcome to instant-riscv, a RISC-V interpreter running in Web Worker!");
    println!("Type :help for help.");
    println!(
        "For more details, please visit the repository at https://github.com/xxuejie/instant-riscv"
    );

    let core_machine = DefaultCoreMachine::<u64, WXorXMemory<SparseMemory<u64>>>::new(
        ISA_IMC | ISA_B | ISA_V,
        VERSION1,
        u64::max_value(),
    );
    let mut machine = DefaultMachineBuilder::new(core_machine).build();

    loop {
        let val = linenoise::input(">> ");
        match val {
            Some(line) => {
                run(&mut machine, &line);
            }
            None => break,
        }
    }
}

fn run<Mac: Machine + core::fmt::Display>(m: &mut Mac, line: &str) {
    if line.starts_with(":") {
        let commands: Vec<&str> = line[1..].split(" ").collect();
        let mut processed = false;
        match commands[0] {
            "reg" => {
                println!("{}", m);
                processed = true;
            }
            "mem" if commands.len() == 3 => {
                if let (Some(addr), Some(len)) =
                    (parse_number(commands[1]), parse_number(commands[2]))
                {
                    match m.memory_mut().load_bytes(addr, len) {
                        Ok(data) => {
                            print_binary(addr, &data, 16);
                            processed = true;
                        }
                        Err(e) => {
                            println!(
                                "Error loading memory at {:x}, length: {:x}: {:?}",
                                addr, len, e
                            );
                        }
                    }
                }
            }
            "v" if commands.len() == 4 => {
                if let (Some(reg), Some(sew), Some(n)) = (
                    parse_number(commands[1]),
                    parse_number(commands[2]),
                    parse_number(commands[3]),
                ) {
                    let data = m.element_ref(reg as usize, sew, n as usize);
                    print_binary(0, &data, 4);
                    processed = true;
                }
            }
            _ => (),
        }
        if !processed {
            print_usage();
        }
    } else {
        match parse::<u64>(line) {
            Ok(insts) => {
                for inst in insts {
                    if let Err(e) = execute_instruction(inst.clone().into(), m) {
                        println!("Error executing instruction: {:?}: {}", inst, e);
                    }
                }
                m.commit_pc();
            }
            Err(e) => println!("Error parsing instruction: {}", e),
        }
    }
}

fn print_binary(start: u64, data: &[u8], addr_width: usize) {
    let mut i = 0;
    while i < data.len() {
        let batch = if i + 16 <= data.len() {
            &data[i..i + 16]
        } else {
            &data[i..data.len()]
        };
        let content = batch
            .iter()
            .map(|v| format!("{:02x}", v))
            .collect::<Vec<String>>()
            .join(" ");
        println!(
            "{:0width$x}: {}",
            start + i as u64,
            content,
            width = addr_width
        );
        i += batch.len();
    }
}

fn parse_number(s: &str) -> Option<u64> {
    if let Some(n) = s.strip_prefix("0x") {
        u64::from_str_radix(n, 16)
    } else if let Some(n) = s.strip_prefix("0b") {
        u64::from_str_radix(n, 2)
    } else if let Some(n) = s.strip_prefix("0o") {
        u64::from_str_radix(n, 8)
    } else if s.starts_with('-') {
        i64::from_str(s).map(|n| n as u64)
    } else {
        u64::from_str(s)
    }
    .ok()
}

fn print_usage() {
    println!(":help => print this help section.");
    println!(":reg => list all RISC-V general registers.");
    println!(":mem <addr> <len> => dump memory data for a certain range.");
    println!(":v <reg> <sew> <n> => dump vector register data for a certain range.");
    println!(
        "Inputs that do not start with : will be interpreted as RISC-V instructions to execute."
    );
}
