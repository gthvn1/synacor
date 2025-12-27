use crate::emulator::Cpu;
use std::fmt;

enum InsnName {
    Add,
    And,
    Call,
    Eq,
    Gt,
    Halt,
    In,
    Jf,
    Jmp,
    Jt,
    Mod,
    Mult,
    Noop,
    Not,
    Or,
    Out,
    Pop,
    Push,
    Ret,
    Rmem,
    Set,
    Wmem,
}

impl InsnName {
    fn of_int(id: u16) -> Self {
        match id {
            0 => InsnName::Halt,
            1 => InsnName::Set,
            2 => InsnName::Push,
            3 => InsnName::Pop,
            4 => InsnName::Eq,
            5 => InsnName::Gt,
            6 => InsnName::Jmp,
            7 => InsnName::Jt,
            8 => InsnName::Jf,
            9 => InsnName::Add,
            10 => InsnName::Mult,
            11 => InsnName::Mod,
            12 => InsnName::And,
            13 => InsnName::Or,
            14 => InsnName::Not,
            15 => InsnName::Rmem,
            16 => InsnName::Wmem,
            17 => InsnName::Call,
            18 => InsnName::Ret,
            19 => InsnName::Out,
            20 => InsnName::In,
            21 => InsnName::Noop,
            _ => panic!("Unknown opcode: {id}"),
        }
    }

    fn arity(&self) -> usize {
        match self {
            InsnName::Halt => 0,
            InsnName::Set => 2,
            InsnName::Push => 1,
            InsnName::Pop => 1,
            InsnName::Eq => 3,
            InsnName::Gt => 3,
            InsnName::Jmp => 1,
            InsnName::Jt => 2,
            InsnName::Jf => 2,
            InsnName::Add => 3,
            InsnName::Mult => 3,
            InsnName::Mod => 3,
            InsnName::And => 3,
            InsnName::Or => 3,
            InsnName::Not => 2,
            InsnName::Rmem => 2,
            InsnName::Wmem => 2,
            InsnName::Call => 1,
            InsnName::Ret => 0,
            InsnName::Out => 1,
            InsnName::In => 1,
            InsnName::Noop => 0,
        }
    }

    pub fn gen_insn(&self, cpu: &mut Cpu) -> Insn {
        let arity = self.arity();
        let args: Vec<u16> = (0..arity).map(|_| cpu.fetch()).collect();
        match self {
            InsnName::Halt => Insn::Halt,
            InsnName::Set => Insn::Set(args[0], args[1]),
            InsnName::Push => Insn::Push(args[0]),
            InsnName::Pop => Insn::Pop(args[0]),
            InsnName::Eq => Insn::Eq(args[0], args[1], args[2]),
            InsnName::Gt => Insn::Gt(args[0], args[1], args[2]),
            InsnName::Jmp => Insn::Jmp(args[0]),
            InsnName::Jt => Insn::Jt(args[0], args[1]),
            InsnName::Jf => Insn::Jf(args[0], args[1]),
            InsnName::Add => Insn::Add(args[0], args[1], args[2]),
            InsnName::Mult => Insn::Mult(args[0], args[1], args[2]),
            InsnName::Mod => Insn::Mod(args[0], args[1], args[2]),
            InsnName::And => Insn::And(args[0], args[1], args[2]),
            InsnName::Or => Insn::Or(args[0], args[1], args[2]),
            InsnName::Not => Insn::Not(args[0], args[1]),
            InsnName::Rmem => Insn::Rmem(args[0], args[1]),
            InsnName::Wmem => Insn::Wmem(args[0], args[1]),
            InsnName::Call => Insn::Call(args[0]),
            InsnName::Ret => Insn::Ret,
            InsnName::Out => {
                Insn::Out(std::char::from_u32(args[0] as u32).expect("failed to convert char"))
            }
            InsnName::In => {
                Insn::In(std::char::from_u32(args[0] as u32).expect("failed to convert char"))
            }
            InsnName::Noop => Insn::Noop,
        }
    }
}

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

pub fn get(cpu: &mut Cpu) -> Insn {
    let opcode = cpu.fetch();
    InsnName::of_int(opcode).gen_insn(cpu)
}
