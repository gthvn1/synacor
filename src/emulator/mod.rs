mod insn;

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
            println!("Breakpoint set at {:05} (0x{:05x})", addr, addr);
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
    fn read(&self, addr: u16) -> u16 {
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

        let start = self.ip.saturating_sub(5).max(layout::MEM_MIN);
        let end = self.ip.saturating_add(5).min(layout::MEM_MAX);

        for addr in start..=end {
            if addr == self.ip {
                out.push_str(&format!(
                    "=> Mem[{:05} (0x{:05X})]: 0x{:05x}\n",
                    addr,
                    addr,
                    self.read(addr.try_into().unwrap())
                ));
            } else {
                out.push_str(&format!(
                    "   Mem[{:05} (0x{:05X})]: 0x{:05x}\n",
                    addr,
                    addr,
                    self.read(addr.try_into().unwrap())
                ));
            }
        }

        out
    }

    pub fn halt(&mut self, reason: &str) {
        println!("CPU halted: {}", reason);
        self.state = State::Stopped;
    }

    pub fn step(&mut self) {
        let Some(insn) = insn::get(self) else { return };
        match insn {
            insn::Insn::Add(a, b, c) => self.halt("instruction not implemented: add"),
            insn::Insn::And(a, b, c) => self.halt("instruction not implemented: and"),
            insn::Insn::Call(a) => self.halt("instruction not implemented: call"),
            insn::Insn::Eq(a, b, c) => self.halt("instruction not implemented: eq"),
            insn::Insn::Gt(a, b, c) => self.halt("instruction not implemented: gt"),
            insn::Insn::Halt => self.halt("Reached Halt instruction"),
            insn::Insn::In(a) => self.halt("instruction not implemented: in"),
            insn::Insn::Jmp(a) => {
                // Not sure that address to jmp can be register
                let value = self.resolve_addr(a);
                self.set_ip(value)
            }
            insn::Insn::Jt(a, b) => {
                let cond = self.resolve_addr(a);
                if cond != 0 {
                    // Not sure that address to jmp can be register
                    let addr = self.resolve_addr(b);
                    self.set_ip(addr)
                }
            }
            insn::Insn::Jf(a, b) => {
                let cond = self.resolve_addr(a);
                if cond == 0 {
                    // Not sure that address to jmp can be register
                    let addr = self.resolve_addr(b);
                    self.set_ip(addr)
                }
            }
            insn::Insn::Mod(a, b, c) => self.halt("instruction not implemented: mod "),
            insn::Insn::Mult(a, b, c) => self.halt("instruction not implemented: mult "),
            insn::Insn::Noop => {}
            insn::Insn::Not(a, b) => self.halt("instruction not implemented: not"),
            insn::Insn::Or(a, b, c) => self.halt("instruction not implemented: or"),
            insn::Insn::Out(a) => print!("{a}"),
            insn::Insn::Pop(a) => self.halt("instruction not implemented: pop"),
            insn::Insn::Push(a) => self.halt("instruction not implemented: push"),
            insn::Insn::Ret => self.halt("instruction not implemented: ret"),
            insn::Insn::Rmem(a, b) => self.halt("instruction not implemented: rmem"),
            insn::Insn::Wmem(a, b) => self.halt("instruction not implemented: wmem"),
            insn::Insn::Set(a, b) => self.write(a, b),
        }
    }

    pub fn cont(&mut self) {
        self.state = State::Running;

        while self.state == State::Running {
            self.step();
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
            print!("Mem[{:05} (0x{:05x})]", self.ip, self.ip);
            if let Some(insn) = insn::get(self) {
                println!("-> {}", insn);
            } else {
                println!("-> <skipped>");
            }
        }
    }

    pub fn run(&mut self) {
        self.reset();
        self.cont();
    }
}
