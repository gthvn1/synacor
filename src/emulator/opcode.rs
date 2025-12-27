use crate::emulator::{Cpu, layout};
use std::fmt;

pub enum Insn {
    Noop,
}

impl fmt::Display for Insn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Insn::Noop => write!(f, "Noop"),
        }
    }
}
pub fn get(cpu: &mut Cpu) -> Insn {
    assert!(cpu.ip <= layout::MEM_MAX);
    match cpu.mem[cpu.ip] {
        0 => todo!("return Halt instruction"),
        1 => todo!("return Set instruction"),
        2 => todo!("return Push instruction"),
        3 => todo!("return Pop instruction"),
        4 => todo!("return Eq instruction"),
        5 => todo!("return Gt instruction"),
        6 => todo!("return Jmp instruction"),
        7 => todo!("return Jt instruction"),
        8 => todo!("return Jf instruction"),
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
        19 => todo!("return Out instruction"),
        20 => todo!("return In instruction"),
        21 => {
            cpu.ip += 1;
            Insn::Noop
        }
        _ => panic!("Unknown opcode: {}", cpu.mem[cpu.ip]),
    }
}
