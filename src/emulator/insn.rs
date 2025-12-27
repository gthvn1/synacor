use crate::emulator::Cpu;
use std::fmt;

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

// Helper to fetch two or three bytes from memory
fn fetch_two(cpu: &mut Cpu) -> (u16, u16) {
    let a = cpu.fetch();
    let b = cpu.fetch();
    (a, b)
}

fn fetch_three(cpu: &mut Cpu) -> (u16, u16, u16) {
    let a = cpu.fetch();
    let b = cpu.fetch();
    let c = cpu.fetch();
    (a, b, c)
}

pub fn get(cpu: &mut Cpu) -> Insn {
    match cpu.fetch() {
        0 => Insn::Halt,
        1 => {
            let (reg, value) = fetch_two(cpu);
            Insn::Set(reg, value)
        }
        2 => {
            let a = cpu.fetch();
            Insn::Push(a)
        }
        3 => {
            let a = cpu.fetch();
            Insn::Pop(a)
        }
        4 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Eq(a, b, c)
        }
        5 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Gt(a, b, c)
        }
        6 => {
            let addr = cpu.fetch();
            Insn::Jmp(addr)
        }
        7 => {
            let (cond, addr) = fetch_two(cpu);
            Insn::Jt(cond, addr)
        }
        8 => {
            let (cond, addr) = fetch_two(cpu);
            Insn::Jf(cond, addr)
        }
        9 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Add(a, b, c)
        }
        10 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Mult(a, b, c)
        }
        11 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Mod(a, b, c)
        }
        12 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::And(a, b, c)
        }
        13 => {
            let (a, b, c) = fetch_three(cpu);
            Insn::Or(a, b, c)
        }
        14 => {
            let (a, b) = fetch_two(cpu);
            Insn::Not(a, b)
        }
        15 => {
            let (a, b) = fetch_two(cpu);
            Insn::Rmem(a, b)
        }
        16 => {
            let (a, b) = fetch_two(cpu);
            Insn::Wmem(a, b)
        }
        17 => {
            let a = cpu.fetch();
            Insn::Call(a)
        }
        18 => Insn::Ret,

        19 => {
            let c = cpu.fetch();
            Insn::Out(std::char::from_u32(c as u32).expect("failed to convert char"))
        }
        20 => {
            todo!("Read character from input")
        }
        21 => Insn::Noop,
        _ => panic!("Unknown opcode: {}", cpu.mem[cpu.ip]),
    }
}
