mod insn;

#[allow(unused)]
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
        assert!(footprint <= layout::MEM_SIZE);

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
        if (layout::MEM_MIN..=layout::MEM_MAX).contains(&(addr as usize)) {
            self.breakpoint = Some(addr);
            println!("Breakpoint set at {:05} (0x{:05x})", addr, addr);
        } else {
            println!("Failed to set addr, {addr} is not in memory");
        }
    }

    // Read the value at the given address
    fn read(&self, addr: u16) -> u16 {
        let addr = addr as usize;
        assert!((layout::MEM_MIN..=layout::REG_MAX).contains(&addr));
        if addr <= layout::MEM_MAX {
            // It is an immediate
            self.mem[addr]
        } else {
            // It is a register
            let reg_id = addr - layout::REG_MIN;
            self.regs[reg_id]
        }
    }

    fn reset(&mut self) {
        self.regs.fill(0);
        self.ip = 0;
    }

    fn fetch(&mut self) -> u16 {
        assert!(self.ip <= layout::MEM_MAX);
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

    pub fn step(&mut self) {
        match insn::get(self) {
            insn::Insn::Add(a, b, c) => todo!("Execute add"),
            insn::Insn::And(a, b, c) => todo!("Execute and"),
            insn::Insn::Call(a) => todo!("Execute call"),
            insn::Insn::Eq(a, b, c) => todo!("Execute eq"),
            insn::Insn::Gt(a, b, c) => todo!("Execute gt"),
            insn::Insn::Halt => self.state = State::Stopped,
            insn::Insn::In(a) => todo!("Execute in"),
            insn::Insn::Jmp(a) => self.set_ip(a),
            insn::Insn::Jt(a, b) => {
                let value = self.read(a);
                if value != 0 {
                    self.set_ip(b)
                }
            }
            insn::Insn::Jf(a, b) => {
                let value = self.read(a);
                if value == 0 {
                    self.set_ip(b)
                }
            }
            insn::Insn::Mod(a, b, c) => todo!("Execute mod "),
            insn::Insn::Mult(a, b, c) => todo!("Execute mult "),
            insn::Insn::Noop => {}
            insn::Insn::Not(a, b) => todo!("Execute not"),
            insn::Insn::Or(a, b, c) => todo!("Execute or"),
            insn::Insn::Out(a) => print!("{a}"),
            insn::Insn::Pop(a) => todo!("Execute pop"),
            insn::Insn::Push(a) => todo!("Execute push"),
            insn::Insn::Ret => todo!("Execute ret"),
            insn::Insn::Rmem(a, b) => todo!("Execute rmem"),
            insn::Insn::Wmem(a, b) => todo!("Execute wmem"),
            insn::Insn::Set(a, b) => todo!("Execute set"),
        }
    }

    pub fn cont(&mut self) {
        self.state = State::Running;

        while self.state == State::Running {
            self.step();
            // Check if there is a breakpoint
            if let Some(bp) = self.breakpoint {
                if bp as usize == self.ip {
                    println!("reached breakpoints at {bp}");
                    break;
                };
            }
        }
    }

    pub fn run(&mut self) {
        self.reset();
        self.cont();
    }
}
