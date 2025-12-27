mod opcode;

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

pub struct Cpu {
    pub mem: [u16; layout::MEM_SIZE], // The size will depend of the ROMs
    pub regs: [u16; layout::NUM_REGS],
    pub ip: usize,      // Instruction pointer
    pub footprint: u16, // keep the program's memory footprint
}

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
        assert!(roms.len() % 2 == 0, "ROMs size is odd");
        let footprint = roms.len() / 2;
        assert!(footprint <= layout::MEM_SIZE);

        let mut cpu = Cpu {
            mem: [0; layout::MEM_SIZE],
            regs: [0; layout::NUM_REGS],
            ip: 0,
            footprint: footprint as u16,
        };

        for (idx, chunk) in roms.chunks_exact(2).enumerate() {
            let low = chunk[0] as u16;
            let high = (chunk[1] as u16) << 8;
            cpu.mem[idx] = high | low;
        }

        cpu
    }

    pub fn step(&mut self) {
        // First we need to read the opcode
        let insn = opcode::get(self);
        println!("step: {}", insn);
    }
}
