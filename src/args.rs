use std::env;

#[derive(Debug)]
pub struct Args {
    pub breakpoint: Option<u16>,
    pub disassemble: bool,
    pub filename: String,
    pub verbose: bool,
}

pub fn read_args() -> Args {
    let mut args = env::args();

    let prog_name = args.next().unwrap();

    let mut breakpoint = None;
    let mut filename = None;
    let mut disassemble = false;
    let mut verbose = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--break" => {
                let line: u16 = args
                    .next()
                    .expect("Expected a number after --break")
                    .parse::<u16>()
                    .expect("Failed to parse breakpoint");
                breakpoint = Some(line);
            }
            "--disassemble" => disassemble = true,
            "--help" => {
                print_help(&prog_name);
                std::process::exit(0);
            }
            "--verbose" => verbose = true,
            _ => {
                if filename.is_some() {
                    println!("filename already set");
                    print_help(&prog_name);
                    std::process::exit(1)
                }
                filename = Some(arg.to_string())
            }
        }
    }

    if let Some(fname) = filename {
        Args {
            breakpoint,
            disassemble,
            filename: fname,
            verbose,
        }
    } else {
        println!("filename not set");
        print_help(&prog_name);
        std::process::exit(1)
    }
}

fn print_help(name: &str) {
    println!("Usage: {name} [--break line] [--disassemble] <filename>");
    println!();
    println!("Options:");
    println!("  --break <line>    Set a breakpoint at the given line number");
    println!("  --disassemble     Print disassemble code from <filename> to stdout");
    println!("  --verbose         Print debug message like the executed opcodes");
    println!("  -h, --help        Print this help message");
}
