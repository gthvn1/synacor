use std::fs::File;
use std::io::{self, Read, Write};

mod args;
mod emulator;

fn main() -> io::Result<()> {
    let args = args::read_args();

    let mut f = File::open(&args.filename)?;
    let mut data = vec![];
    f.read_to_end(&mut data)?;

    let mut cpu = emulator::Cpu::load(data);
    println!(
        "{} words loaded in memory from {}",
        cpu.footprint, &args.filename
    );

    if let Some(bp) = args.breakpoint {
        cpu.set_breakpoint(bp);
    }

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
            "c" => cpu.cont(),
            "p" => println!("{}", cpu.print()),
            "q" => break,
            "r" => cpu.run(),
            "s" => cpu.step(),
            _ => println!("you print {buf}"),
        }
    }

    Ok(())
}
