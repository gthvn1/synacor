mod insn;

macro_rules! vprint {
    ($verbose:expr, $($arg:tt)*) => {
        if $verbose {
            println!($($arg)*);
        }
    };
}

mod layout {
    /// Total number of addressable memory words.
    /// Memory addresses range from 0 to MEM_SIZE - 1.
    pub const MEM_SIZE: usize = 32_768;

    /// Lowest valid memory address.
    pub const MEM_MIN: usize = 0;

    /// Highest valid memory address.
    pub const MEM_MAX: usize = MEM_SIZE - 1;

    /// Number of general-purpose registers.
    pub const NUM_REGS: usize = 8;

    /// Lowest address mapped to a register.
    pub const REG_MIN: usize = MEM_SIZE;

    /// Highest address mapped to a register.
    pub const REG_MAX: usize = REG_MIN + NUM_REGS - 1;

    pub fn is_mem(addr: u16) -> bool {
        (MEM_MIN..=MEM_MAX).contains(&(addr as usize))
    }

    pub fn is_reg(addr: u16) -> bool {
        (REG_MIN..=REG_MAX).contains(&(addr as usize))
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Running,
    Stopped,
}

pub struct Cpu {
    pub mem: [u16; layout::MEM_SIZE], // The size will depend of the ROMs
    pub regs: [u16; layout::NUM_REGS],
    pub stack: Vec<u16>,
    pub ip: usize,      // Instruction pointer
    pub footprint: u16, // keep the program's memory footprint
    state: State,
    breakpoint: Option<u16>,
}

#[allow(unused)]
impl Cpu {
    pub fn load(roms: Vec<u8>) -> Cpu {
        // The format of the ROM is each number is stored as a 16 bit little endian pair
        // Example:
        //   - roms[0] -> low byte of mem[0]
        //   - roms[1] -> high byte of mem[0]
        // ...
        //   - roms[i] -> low byte of mem[i]
        //   - roms[i + 1] -> high byte of mem[i]
        // Programs are loaded into memory starting at address 0
        assert!(roms.len().is_multiple_of(2), "ROMs size is odd");
        let footprint = roms.len() / 2;
        assert!(footprint <= layout::MEM_MAX);

        let mut cpu = Cpu {
            mem: [0; layout::MEM_SIZE],
            regs: [0; layout::NUM_REGS],
            stack: vec![],
            ip: 0,
            footprint: footprint as u16,
            state: State::Stopped,
            breakpoint: None,
        };

        for (idx, chunk) in roms.chunks_exact(2).enumerate() {
            let low = chunk[0] as u16;
            let high = (chunk[1] as u16) << 8;
            cpu.mem[idx] = high | low;
        }

        cpu
    }

    pub fn set_breakpoint(&mut self, addr: u16) {
        if layout::is_mem(addr) {
            self.breakpoint = Some(addr);
            println!("Breakpoint set at {:05} (0x{:04x})", addr, addr);
        } else {
            println!("Failed to set addr, {addr} is not in memory");
        }
    }

    // Resolve the addr, if it is in the memroy range the address is returned
    // and if it is in the register range it is the content of the register that
    // is returned
    fn resolve_addr(&self, addr: u16) -> u16 {
        if layout::is_mem(addr) {
            addr
        } else if layout::is_reg(addr) {
            let reg_id = addr as usize - layout::REG_MIN;
            self.regs[reg_id]
        } else {
            panic!("{addr} is not valid memory");
        }
    }

    // Read the value at the given address. If it is in memory range it returns the content
    // at this address, otherwise it returns the content of the register.
    pub fn read(&self, addr: u16) -> u16 {
        if layout::is_mem(addr) {
            self.mem[addr as usize]
        } else if layout::is_reg(addr) {
            let reg_id = addr as usize - layout::REG_MIN;
            self.regs[reg_id]
        } else {
            panic!("{addr} is not valid memory");
        }
    }

    // Read the value at the given address. If it is in memory range it returns the content
    // at this address, otherwise it returns the content of the register.
    fn write(&mut self, addr: u16, value: u16) {
        if layout::is_reg(addr) {
            let reg_id = addr as usize - layout::REG_MIN;
            self.regs[reg_id] = value;
        } else {
            panic!("{addr} is not a register, only registers are writtable");
        }
    }

    fn reset(&mut self) {
        self.regs.fill(0);
        self.ip = 0;
    }

    fn fetch(&mut self) -> u16 {
        if self.ip > layout::MEM_MAX {
            panic!("IP {} is above max mem {}", self.ip, layout::MEM_MAX);
        }
        let word = self.mem[self.ip];
        self.ip += 1;
        word
    }

    fn set_ip(&mut self, ip: u16) {
        assert!(ip as usize >= layout::MEM_MIN);
        assert!(ip as usize <= layout::MEM_MAX);
        self.ip = ip as usize;
    }

    pub fn print(&self) -> String {
        let mut out = String::new();

        out.push_str("   -- Memory --\n");
        let start = self.ip.saturating_sub(5).max(layout::MEM_MIN);
        let end = self.ip.saturating_add(5).min(layout::MEM_MAX);

        for addr in start..=end {
            if addr == self.ip {
                out.push_str(&format!(
                    "=> Mem[{:05} (0x{:04X})]: 0x{:04x}\n",
                    addr,
                    addr,
                    self.read(addr.try_into().unwrap())
                ));
            } else {
                out.push_str(&format!(
                    "   Mem[{:05} (0x{:04X})]: 0x{:04x}\n",
                    addr,
                    addr,
                    self.read(addr.try_into().unwrap())
                ));
            }
        }

        out.push_str("   -- Registers --\n   ");
        for (idx, reg) in self.regs.iter().enumerate() {
            out.push_str(&format!("[{:1}]:0x{:04x} ", idx, reg));
        }
        out.push_str("\n");

        out
    }

    pub fn halt(&mut self, reason: &str) {
        println!("CPU halted: {}", reason);
        self.state = State::Stopped;
    }

    pub fn step(&mut self, verbose: bool) {
        let Some(insn) = insn::get(self) else { return };
        match insn {
            insn::Insn::Add(a, b, c) => {
                // We are expecting a to be a register, it will be checked
                // when writting
                let valb = self.resolve_addr(b) as usize;
                let valc = self.resolve_addr(c) as usize;
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Add {a} {valb} {valc}",
                    self.ip,
                    self.ip
                );
                self.write(a, u16::try_from((valb + valc) % layout::MEM_SIZE).unwrap());
            }
            insn::Insn::And(a, b, c) => {
                let valb = self.resolve_addr(b);
                let valc = self.resolve_addr(c);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), And {a} {valb} {valc}",
                    self.ip,
                    self.ip
                );
                self.write(a, valb & valc);
            }
            insn::Insn::Call(a) => {
                self.stack.push(u16::try_from(self.ip).unwrap());
                let addr = self.resolve_addr(a);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Call: Push {} on the stack, set ip to {addr}",
                    self.ip,
                    self.ip,
                    self.ip
                );
                self.set_ip(addr)
            }
            insn::Insn::Eq(a, b, c) => {
                let valb = self.resolve_addr(b);
                let valc = self.resolve_addr(c);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Eq: comparing {valb} == {valc}, set 0x{a:04x}",
                    self.ip,
                    self.ip
                );
                if valb == valc {
                    self.write(a, 1);
                } else {
                    self.write(a, 0);
                }
            }
            insn::Insn::Gt(a, b, c) => {
                let valb = self.resolve_addr(b);
                let valc = self.resolve_addr(c);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Gt: comparing {valb} > {valc}, set 0x{a:04x}",
                    self.ip,
                    self.ip
                );
                if valb > valc {
                    self.write(a, 1);
                } else {
                    self.write(a, 0);
                }
            }
            insn::Insn::Halt => self.halt("Reached Halt instruction"),
            insn::Insn::In(a) => self.halt("instruction not implemented: in"),
            insn::Insn::Jmp(a) => {
                // Not sure that address to jmp can be register
                let value = self.resolve_addr(a);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Jmp: set ip to 0x{value:04x}",
                    self.ip,
                    self.ip
                );
                self.set_ip(value)
            }
            insn::Insn::Jt(a, b) => {
                let cond = self.resolve_addr(a);
                if cond != 0 {
                    // Not sure that address to jmp can be register
                    let addr = self.resolve_addr(b);
                    self.set_ip(addr);
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Jt: set ip to 0x{addr:04x} (cond = {cond})",
                        self.ip,
                        self.ip
                    );
                } else {
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Jt: ip not updated (cond = {cond})",
                        self.ip,
                        self.ip
                    );
                }
            }
            insn::Insn::Jf(a, b) => {
                let cond = self.resolve_addr(a);
                if cond == 0 {
                    // Not sure that address to jmp can be register
                    let addr = self.resolve_addr(b);
                    self.set_ip(addr);
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Jf: set ip to 0x{addr:04x} (cond = {cond})",
                        self.ip,
                        self.ip
                    );
                } else {
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Jf: ip not updated (cond = {cond})",
                        self.ip,
                        self.ip
                    );
                }
            }
            insn::Insn::Mod(a, b, c) => {
                let valb = self.resolve_addr(b);
                let valc = self.resolve_addr(c);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Mod: {a} = {}",
                    valb % valc,
                    self.ip,
                    self.ip
                );
                self.write(a, valb % valc);
            }
            insn::Insn::Mult(a, b, c) => {
                let valb = self.resolve_addr(b) as usize;
                let valc = self.resolve_addr(c) as usize;
                let res = u16::try_from((valb * valc) % layout::MEM_SIZE).unwrap();
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Mult: {a} = {res}",
                    self.ip,
                    self.ip
                );
                self.write(a, res);
            }
            insn::Insn::Noop => vprint!(verbose, "IP {:05} (0x{:04x}), Noop", self.ip, self.ip),
            insn::Insn::Not(a, b) => {
                let value = self.resolve_addr(b);
                let res = !value & 0x7FFF;
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Not: {a} = {res}",
                    self.ip,
                    self.ip
                );
                self.write(a, res);
            }
            insn::Insn::Or(a, b, c) => {
                let valb = self.resolve_addr(b);
                let valc = self.resolve_addr(c);
                let res = valb | valc;
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Or: {a} = {res}",
                    self.ip,
                    self.ip
                );
                self.write(a, valb | valc);
            }
            insn::Insn::Out(a) => print!("{a}"),
            insn::Insn::Pop(a) => {
                if let Some(value) = self.stack.pop() {
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Pop: {value}",
                        self.ip,
                        self.ip
                    );
                    self.write(a, value);
                } else {
                    self.halt("cannot pop empty stack");
                }
            }
            insn::Insn::Push(a) => {
                let value = self.resolve_addr(a);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Push: {value}",
                    self.ip,
                    self.ip
                );
                self.stack.push(value);
            }
            insn::Insn::Ret => {
                if let Some(addr) = self.stack.pop() {
                    vprint!(
                        verbose,
                        "IP {:05} (0x{:04x}), Ret: set ip to {addr}",
                        self.ip,
                        self.ip
                    );
                    self.set_ip(addr);
                } else {
                    self.halt("cannot pop empty stack");
                }
            }
            insn::Insn::Rmem(a, b) => {
                let value = self.read(b);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Rmem: read {value} and try to write it at 0x{:04x}",
                    self.ip,
                    self.ip,
                    a
                );
                self.write(a, value);
            }
            insn::Insn::Wmem(a, b) => {
                let value = self.resolve_addr(b);
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Wmem: write {value} into memory at 0x{:04x}",
                    self.ip,
                    self.ip,
                    a
                );
                self.write(a, value);
            }
            insn::Insn::Set(a, b) => {
                vprint!(
                    verbose,
                    "IP {:05} (0x{:04x}), Set: register 0x{a:04x} to 0x{b:04x}",
                    self.ip,
                    self.ip
                );
                self.write(a, b)
            }
        }
    }

    pub fn cont(&mut self, verbose: bool) {
        self.state = State::Running;

        while self.state == State::Running {
            self.step(verbose);
            // Check if there is a breakpoint
            if let Some(bp) = self.breakpoint
                && bp as usize == self.ip
            {
                println!("reached breakpoints at {bp}");
                break;
            }
        }
    }

    pub fn disassemble(&mut self) {
        self.reset();
        let upper = self.footprint as usize;

        println!("Disassemble from {} to {}", layout::MEM_MIN, upper);
        while self.ip <= upper {
            print!("Mem[{:05} (0x{:04x})]", self.ip, self.ip);
            if let Some(insn) = insn::get(self) {
                println!("-> {}", insn);
            } else {
                println!("-> <skipped>");
            }
        }
    }

    pub fn run(&mut self, verbose: bool) {
        self.reset();
        self.cont(verbose);
    }
}
