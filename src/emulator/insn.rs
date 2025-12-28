use crate::emulator::Cpu;
use std::fmt;

#[derive(Debug, Clone, Copy)]
struct OpCode {
    name: &'static str,
    arity: usize, // number of parameters
}

#[rustfmt::skip]
const OPCODES: &[OpCode] = &[
    OpCode {name: "Halt", arity: 0}, // 0
    OpCode {name: "Set" , arity: 2},
    OpCode {name: "Push", arity: 1},
    OpCode {name: "Pop" , arity: 1},
    OpCode {name: "Eq"  , arity: 3},
    OpCode {name: "Gt"  , arity: 3}, // 5
    OpCode {name: "Jmp" , arity: 1},
    OpCode {name: "Jt"  , arity: 2},
    OpCode {name: "Jf"  , arity: 2},
    OpCode {name: "Add" , arity: 3},
    OpCode {name: "Mult", arity: 3}, // 10
    OpCode {name: "Mod" , arity: 3},
    OpCode {name: "And" , arity: 3},
    OpCode {name: "Or"  , arity: 3},
    OpCode {name: "Not" , arity: 2},
    OpCode {name: "Rmem", arity: 2}, // 15
    OpCode {name: "Wmem", arity: 2},
    OpCode {name: "Call", arity: 1},
    OpCode {name: "Ret" , arity: 0},
    OpCode {name: "Out" , arity: 1},
    OpCode {name: "In"  , arity: 1}, // 20
    OpCode {name: "Noop", arity: 0}, // 21
];

pub fn gen_insn(opcode: usize, cpu: &mut Cpu) -> Option<Insn> {
    if opcode >= OPCODES.len() {
        println!(
            "Unknown opcode {opcode} at {:05} (0x{:05x}). CPU halted",
            cpu.ip, cpu.ip
        );
        cpu.halt("unknown opcode");
        return None;
    }

    let op = OPCODES[opcode];
    let args: Vec<u16> = (0..op.arity).map(|_| cpu.fetch()).collect();
    match op.name {
        "Halt" => Some(Insn::Halt),
        "Set" => Some(Insn::Set(args[0], args[1])),
        "Push" => Some(Insn::Push(args[0])),
        "Pop" => Some(Insn::Pop(args[0])),
        "Eq" => Some(Insn::Eq(args[0], args[1], args[2])),
        "Gt" => Some(Insn::Gt(args[0], args[1], args[2])),
        "Jmp" => Some(Insn::Jmp(args[0])),
        "Jt" => Some(Insn::Jt(args[0], args[1])),
        "Jf" => Some(Insn::Jf(args[0], args[1])),
        "Add" => Some(Insn::Add(args[0], args[1], args[2])),
        "Mult" => Some(Insn::Mult(args[0], args[1], args[2])),
        "Mod" => Some(Insn::Mod(args[0], args[1], args[2])),
        "And" => Some(Insn::And(args[0], args[1], args[2])),
        "Or" => Some(Insn::Or(args[0], args[1], args[2])),
        "Not" => Some(Insn::Not(args[0], args[1])),
        "Rmem" => Some(Insn::Rmem(args[0], args[1])),
        "Wmem" => Some(Insn::Wmem(args[0], args[1])),
        "Call" => Some(Insn::Call(args[0])),
        "Ret" => Some(Insn::Ret),
        "Out" => Some(Insn::Out(
            std::char::from_u32(args[0] as u32).expect("failed to convert char"),
        )),
        "In" => Some(Insn::In(
            std::char::from_u32(args[0] as u32).expect("failed to convert char"),
        )),
        "Noop" => Some(Insn::Noop),
        _ => panic!("unreachable"),
    }
}

#[derive(Debug)]
pub enum Insn {
    Add(u16, u16, u16),
    And(u16, u16, u16),
    Call(u16),
    Eq(u16, u16, u16),
    Gt(u16, u16, u16),
    Halt,
    In(char),
    Jf(u16, u16),
    Jmp(u16),
    Jt(u16, u16),
    Mod(u16, u16, u16),
    Mult(u16, u16, u16),
    Noop,
    Not(u16, u16),
    Or(u16, u16, u16),
    Out(char),
    Pop(u16),
    Push(u16),
    Ret,
    Rmem(u16, u16),
    Set(u16, u16),
    Wmem(u16, u16),
}

impl fmt::Display for Insn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Insn::Add(a, b, c) => write!(f, "09: Add   {a:04X} {b:04X} {c:04X}"),
            Insn::And(a, b, c) => write!(f, "0C: And   {a:04X} {b:04X} {c:04X}"),
            Insn::Call(a) => write!(f, "11: Call  {a:04X}"),
            Insn::Eq(a, b, c) => write!(f, "04: Eq    {a:04X} {b:04X} {c:04X}"),
            Insn::Gt(a, b, c) => write!(f, "05: Gt    {a:04X} {b:04X} {c:04X}"),
            Insn::Halt => write!(f, "00: Halt"),
            Insn::In(a) => write!(f, "14: In    <{a}>"),
            Insn::Jmp(a) => write!(f, "06: Jmp   {a:04X}"),
            Insn::Jt(a, b) => write!(f, "07: Jt    {a:04X} {b:04X}"),
            Insn::Jf(a, b) => write!(f, "08: Jf    {a:04X} {b:04X}"),
            Insn::Mod(a, b, c) => write!(f, "0B: Mod   {a:04X} {b:04X} {c:04X}"),
            Insn::Mult(a, b, c) => write!(f, "0A: Mult  {a:04X} {b:04X} {c:04X}"),
            Insn::Noop => write!(f, "15: Noop"),
            Insn::Not(a, b) => write!(f, "0D: Not   {a:04X} {b:04X}"),
            Insn::Or(a, b, c) => write!(f, "Or    {a:04X} {b:04X} {c:04X}"),
            Insn::Out(a) => write!(f, "13: Out   <{a}>"),
            Insn::Pop(a) => write!(f, "03: Pop   {a:04X}"),
            Insn::Push(a) => write!(f, "02: Push  {a:04X}"),
            Insn::Ret => write!(f, "12: Ret"),
            Insn::Rmem(a, b) => write!(f, "0E: Rmem  {a:04X} {b:04X}"),
            Insn::Wmem(a, b) => write!(f, "0F: Wmem  {a:04X} {b:04X}"),
            Insn::Set(a, b) => write!(f, "01: Set   {a:04X} {b:04X}"),
        }
    }
}

pub fn get(cpu: &mut Cpu) -> Option<Insn> {
    let opcode = cpu.fetch();
    gen_insn(opcode as usize, cpu)
}
