use std::fs::File;
use std::io::{self, Read, Write};

mod emulator;

fn main() -> io::Result<()> {
    // TODO: read filename from command line
    let filename = "roms/challenge.bin";
    let mut f = File::open(filename)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;

    let mut cpu = emulator::Cpu::load(data);
    println!("{} words loaded in memory from {}", cpu.footprint, filename);

    // Enter debug mode by default
    // TODO: use a parameter
    println!("[b]reak, [c]ontinue, [p]rint, [q]uit, [r]un, [s]tep");
    loop {
        print!("debug> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read input");
        match buf.trim() {
            "b" => println!("Not implemented"),
            "c" => println!("Not implemented"),
            "p" => println!("Not implemented"),
            "q" => break,
            "r" => {
                cpu.run();
                break;
            }
            "s" => println!("Not implemented"),
            _ => println!("you print {buf}"),
        }
    }

    Ok(())
}
