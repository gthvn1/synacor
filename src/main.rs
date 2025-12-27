use std::fs::File;
use std::io::{self, Read};

mod emulator;

fn main() -> io::Result<()> {
    let mut f = File::open("roms/challenge.bin")?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;

    // Print 16 first bytes of data
    for byte in data.iter().take(16) {
        print!("{:02X}  ", byte);
    }
    println!();

    let mut cpu = emulator::Cpu::load(data);
    // Print 8 first bytes of memory
    for (idx, byte) in cpu.mem.iter().take(8).enumerate() {
        println!("{:04X}: {:02X}  ", idx, byte);
    }

    println!("{} words loaded in memory", cpu.footprint);

    cpu.disassemble();
    cpu.run();
    Ok(())
}
