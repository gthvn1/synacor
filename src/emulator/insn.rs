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
    match cpu.fetch() {
        0 => Insn::Halt,
        1 => todo!("return Set instruction"),
        2 => todo!("return Push instruction"),
        3 => todo!("return Pop instruction"),
        4 => todo!("return Eq instruction"),
        5 => todo!("return Gt instruction"),
        6 => {
            let addr = cpu.fetch();
            Insn::Jmp(addr as usize)
        }
        7 => {
            let cond = cpu.fetch();
            let addr = cpu.fetch();
            Insn::Jt(cond as usize, addr as usize)
        }
        8 => {
            let cond = cpu.fetch();
            let addr = cpu.fetch();
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
            let c = cpu.fetch();
            Insn::Out(std::char::from_u32(c as u32).expect("failed to convert char"))
        }
        20 => todo!("return In instruction"),
        21 => Insn::Noop,
        _ => panic!("Unknown opcode: {}", cpu.mem[cpu.ip]),
    }
}
