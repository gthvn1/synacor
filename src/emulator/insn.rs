use crate::emulator::{Cpu, layout};
use std::fmt;

pub enum Insn {
    Halt,
    Jmp(usize),
    Jt(usize, usize),
    Jf(usize, usize),
    Noop,
    Out(char),
}

impl fmt::Display for Insn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Insn::Halt => write!(f, "Halt"),
            Insn::Jmp(addr) => write!(f, "Jmp {addr:05}"),
            Insn::Jt(cond, addr) => write!(f, "Jt {cond} {addr:05}"),
            Insn::Jf(cond, addr) => write!(f, "Jf {cond} {addr:05}"),
            Insn::Noop => write!(f, "Noop"),
            Insn::Out(c) => write!(f, "Out <{c}>"),
        }
    }
}
pub fn get(cpu: &mut Cpu) -> Insn {
    assert!(cpu.ip <= layout::MEM_MAX);
    match cpu.mem[cpu.ip] {
        0 => {
            cpu.ip += 1;
            Insn::Halt
        }
        1 => todo!("return Set instruction"),
        2 => todo!("return Push instruction"),
        3 => todo!("return Pop instruction"),
        4 => todo!("return Eq instruction"),
        5 => todo!("return Gt instruction"),
        6 => {
            let addr = cpu.mem[cpu.ip + 1];
            cpu.ip += 2;
            Insn::Jmp(addr as usize)
        }
        7 => {
            let cond = cpu.mem[cpu.ip + 1];
            let addr = cpu.mem[cpu.ip + 2];
            cpu.ip += 3;
            Insn::Jt(cond as usize, addr as usize)
        }
        8 => {
            let cond = cpu.mem[cpu.ip + 1];
            let addr = cpu.mem[cpu.ip + 2];
            cpu.ip += 3;
            Insn::Jf(cond as usize, addr as usize)
        }
        9 => todo!("return Add instruction"),
        10 => todo!("return Mult instruction"),
        11 => todo!("return Mod instruction"),
        12 => todo!("return And instruction"),
        13 => todo!("return Or instruction"),
        14 => todo!("return Not instruction"),
        15 => todo!("return Rmem instruction"),
        16 => todo!("return Wmem instruction"),
        17 => todo!("return Call instruction"),
        18 => todo!("return Ret instruction"),
        19 => {
            let c = cpu.mem[cpu.ip + 1];
            cpu.ip += 2;
            Insn::Out(std::char::from_u32(c as u32).expect("failed to convert char"))
        }
        20 => todo!("return In instruction"),
        21 => {
            cpu.ip += 1;
            Insn::Noop
        }
        _ => panic!("Unknown opcode: {}", cpu.mem[cpu.ip]),
    }
}
