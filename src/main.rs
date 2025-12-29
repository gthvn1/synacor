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

    if args.disassemble {
        cpu.disassemble();
        println!("--- Done ---");
        std::process::exit(0);
    }

    if let Some(bp) = args.breakpoint {
        cpu.set_breakpoint(bp);
    }

    // Enter debug mode by default
    // TODO: use a parameter

    println!("[b]reak, [c]ontinue, [p]rint, [q]uit, read, [r]un, [s]tep");
    loop {
        print!("debug> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let input = input.trim();
        let mut parts = input.split_whitespace();

        match parts.next() {
            Some("b") | Some("break") => {
                if let Some(arg) = parts.next() {
                    match arg.parse::<u16>() {
                        Ok(n) => cpu.set_breakpoint(n),
                        Err(_) => {
                            println!("Invalid breakpoint, a valid memory integer is expected")
                        }
                    }
                } else {
                    println!("Missing breakpoint integer")
                }
            }
            Some("c") | Some("continue") => cpu.cont(args.verbose),
            Some("p") | Some("print") => println!("{}", cpu.print()),
            Some("read") => {
                if let Some(arg) = parts.next() {
                    match arg.parse::<u16>() {
                        Ok(n) => println!("[0x{n:04x}] => {}", cpu.read(n)),
                        Err(_) => {
                            println!("Invalid memory address")
                        }
                    }
                } else {
                    println!("address is missing")
                }
            }
            Some("r") | Some("run") => cpu.run(args.verbose),
            Some("q") | Some("quit") => break,
            Some("s") | Some("step") => cpu.step(args.verbose),
            _ => println!("Unknown input"),
        }
    }

    Ok(())
}
